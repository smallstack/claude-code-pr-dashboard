<script lang="ts">
	
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { FitAddon } from "@xterm/addon-fit";
	import { Terminal } from "@xterm/xterm";
import { onDestroy, onMount } from "svelte";
	import "@xterm/xterm/css/xterm.css";
	import { prStore } from "../stores/prs.svelte";
	import { sessionStore } from "../stores/sessions.svelte";
	import { settingsStore } from "../stores/settings.svelte";
	import type { Session } from "../types";

	interface Props {
		session: Session;
	}

	let { session }: Props = $props();
	const justPassed = $derived(session.pr ? prStore.recentlyPassed.has(session.pr.number) : false);

	let terminalEl: HTMLDivElement;
	let terminal: Terminal;
	let fitAddon: FitAddon;
	let unlisten: (() => void) | null = null;
	let resizeObserver: ResizeObserver | null = null;

	onMount(async () => {
		// Create xterm instance
		terminal = new Terminal({
			cursorBlink: true,
			fontSize: 13,
			fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace",
			theme: {
				background: "#1a1b26",
				foreground: "#c0caf5",
				cursor: "#c0caf5",
				selectionBackground: "#33467c",
				black: "#15161e",
				red: "#f7768e",
				green: "#9ece6a",
				yellow: "#e0af68",
				blue: "#7aa2f7",
				magenta: "#bb9af7",
				cyan: "#7dcfff",
				white: "#a9b1d6"
			}
		});

		fitAddon = new FitAddon();
		terminal.loadAddon(fitAddon);
		terminal.open(terminalEl);

		// Fit after a brief delay to ensure DOM is ready
		requestAnimationFrame(() => {
			fitAddon.fit();
		});

		const cols = terminal.cols;
		const rows = terminal.rows;

		// Track whether we need to auto-send /fix-pr once Claude is ready
		let pendingFixPr = session.status === "fixing";

		// Listen for PTY output
		// xterm.js handles magnetic auto-scroll natively:
		// auto-scrolls on new output, pauses when user scrolls up, resumes at bottom
		const unlistenFn = await listen<string>(`pty-output-${session.id}`, (event) => {
			terminal.write(event.payload);

			// Detect when Claude CLI is ready (shows the > prompt) and send /fix-pr
			if (pendingFixPr && event.payload.includes("❯")) {
				pendingFixPr = false;
				setTimeout(() => {
					invoke("write_session", { id: session.id, data: "/fix-pr\r" }).catch(console.error);
				}, 500);
			}
		});
		unlisten = unlistenFn;

		// Forward terminal input to PTY
		terminal.onData((data) => {
			invoke("write_session", { id: session.id, data }).catch(console.error);
		});

		// Spawn the session first, before setting up resize observer
		const dockerArgs = {
			githubToken: settingsStore.githubToken,
			claudeCredentialsPath: settingsStore.claudeCredentialsPath,
			claudeModel: settingsStore.claudeModel,
			gitUserName: settingsStore.gitUserName,
			gitUserEmail: settingsStore.gitUserEmail,
		};

		if (session.pr && session.status === "fixing") {
			await invoke("auto_fix_pr", {
				id: session.id,
				repo: prStore.repo,
				branch: session.pr.headRefName,
				...dockerArgs,
				cols,
				rows
			});
		} else if (session.pr) {
			await invoke("open_claude", {
				id: session.id,
				repo: prStore.repo,
				...dockerArgs,
				cols,
				rows
			});
		} else {
			await invoke("create_session", {
				id: session.id,
				cols,
				rows
			});
		}

		// Set up resize observer only after session exists
		resizeObserver = new ResizeObserver(() => {
			fitAddon.fit();
			invoke("resize_session", {
				id: session.id,
				cols: terminal.cols,
				rows: terminal.rows
			}).catch(console.error);
		});
		resizeObserver.observe(terminalEl);
	});

	onDestroy(() => {
		unlisten?.();
		resizeObserver?.disconnect();
		terminal?.dispose();
		invoke("close_session", { id: session.id }).catch(console.error);
	});

	function handleClose() {
		sessionStore.removeSession(session.id);
	}

	function handleAutoFix() {
		invoke("write_session", { id: session.id, data: "/fix-pr\r" }).catch(console.error);
		sessionStore.updateStatus(session.id, "fixing");
	}
</script>

<div class="session-panel" class:panel-just-passed={justPassed}>
	<div class="panel-header">
		<span class="panel-label">
			{#if session.status === "fixing"}
				<span class="fixing-indicator"></span>
			{/if}
			{session.label}
		</span>
		<div class="panel-actions">
			{#if session.pr}
				<button class="header-btn" onclick={handleAutoFix} title="Send /fix-pr">Fix</button>
				<a
					class="header-btn"
					href={session.pr.url}
					target="_blank"
					rel="noopener noreferrer"
					title="Open PR in browser"
				>PR</a>
			{/if}
			<button class="close-btn" onclick={handleClose} title="Close session">x</button>
		</div>
	</div>
	<div class="terminal-container" bind:this={terminalEl}></div>
</div>

<style>
	.session-panel {
		flex: 1;
		min-width: 0;
		min-height: 0;
		display: flex;
		flex-direction: column;
		background: var(--bg-primary);
	}

	.panel-just-passed {
		animation: panel-highlight-flash 2s ease-out;
	}

	@keyframes panel-highlight-flash {
		0% { box-shadow: inset 0 0 0 2px var(--success); }
		100% { box-shadow: inset 0 0 0 2px transparent; }
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.35rem 0.75rem;
		background: var(--bg-secondary);
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
	}

	.panel-label {
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--text-secondary);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.fixing-indicator {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--warning);
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.3; }
	}

	.panel-actions {
		display: flex;
		align-items: center;
		gap: 0.35rem;
	}

	.header-btn {
		padding: 0.1rem 0.4rem;
		font-size: 0.75rem;
		background: var(--bg-tertiary);
		color: var(--text-muted);
		border: 1px solid var(--border);
		border-radius: 3px;
		text-decoration: none;
		line-height: 1.3;
	}

	.header-btn:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.close-btn {
		padding: 0 0.4rem;
		background: none;
		border: none;
		color: var(--text-muted);
		font-size: 0.9rem;
		line-height: 1;
		border-radius: 3px;
	}

	.close-btn:hover {
		background: var(--bg-hover);
		color: var(--error);
	}

	.terminal-container {
		flex: 1;
		min-height: 0;
		padding: 4px;
		overflow: hidden;
	}

	/* xterm.js styles override */
	.terminal-container :global(.xterm) {
		height: 100%;
	}

	.terminal-container :global(.xterm-viewport) {
		overflow-y: auto !important;
	}
</style>
