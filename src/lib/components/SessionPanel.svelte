<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { Terminal } from "@xterm/xterm";
	import { FitAddon } from "@xterm/addon-fit";
	import { sessionStore } from "../stores/sessions.svelte";
	import { prStore } from "../stores/prs.svelte";
	import type { Session } from "../types";

	interface Props {
		session: Session;
	}

	let { session }: Props = $props();

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

		// Create PTY session with home directory as default cwd
		const cwd = "~";
		await invoke("create_session", {
			id: session.id,
			cwd,
			cols,
			rows
		});

		// Listen for PTY output
		const unlistenFn = await listen<string>(`pty-output-${session.id}`, (event) => {
			terminal.write(event.payload);
		});
		unlisten = unlistenFn;

		// Forward terminal input to PTY
		terminal.onData((data) => {
			invoke("write_session", { id: session.id, data }).catch(console.error);
		});

		// Handle resize
		resizeObserver = new ResizeObserver(() => {
			fitAddon.fit();
			invoke("resize_session", {
				id: session.id,
				cols: terminal.cols,
				rows: terminal.rows
			}).catch(console.error);
		});
		resizeObserver.observe(terminalEl);

		// If this session has a PR and is in "fixing" mode, run auto_fix_pr
		if (session.pr && session.status === "fixing") {
			// Small delay to let the shell initialize
			setTimeout(async () => {
				try {
					await invoke("auto_fix_pr", {
						id: session.id,
						repoPath: ".",
						branch: session.pr!.headRefName
					});
				} catch (e) {
					console.error("auto_fix_pr failed:", e);
				}
			}, 1000);
		} else if (session.pr) {
			// Open Claude in the PR context
			setTimeout(async () => {
				try {
					await invoke("open_claude", {
						id: session.id,
						repoPath: "."
					});
				} catch (e) {
					console.error("open_claude failed:", e);
				}
			}, 500);
		}
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
</script>

<div class="session-panel">
	<div class="panel-header">
		<span class="panel-label">
			{#if session.status === "fixing"}
				<span class="fixing-indicator"></span>
			{/if}
			{session.label}
		</span>
		<button class="close-btn" onclick={handleClose} title="Close session">x</button>
	</div>
	<div class="terminal-container" bind:this={terminalEl}></div>
</div>

<style>
	.session-panel {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		background: var(--bg-primary);
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
