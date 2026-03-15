<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { prStore } from "../stores/prs.svelte";
	import { sessionStore } from "../stores/sessions.svelte";
	import { settingsStore } from "../stores/settings.svelte";
	import type { PullRequest } from "../types";

	const INTERVAL = 60;
	let repoInput = $state(prStore.repo);
	let showSettings = $state(!settingsStore.isConfigured);
	let autoUpdate = $state(true);
	let countdown = $state(INTERVAL);
	let hasLoaded = $state(false);
	let timer: ReturnType<typeof setInterval> | null = null;

	function startTimer() {
		stopTimer();
		countdown = INTERVAL;
		timer = setInterval(() => {
			countdown--;
			if (countdown <= 0) {
				void prStore.fetchPrs();
				countdown = INTERVAL;
			}
		}, 1000);
	}

	function stopTimer() {
		if (timer) {
			clearInterval(timer);
			timer = null;
		}
	}

	$effect(() => {
		if (autoUpdate && hasLoaded) {
			startTimer();
		} else {
			stopTimer();
		}
		return () => stopTimer();
	});

	async function handleRepoSubmit() {
		prStore.setRepo(repoInput.trim());
		await prStore.fetchPrs();
		hasLoaded = true;
	}

	function handleOpenCli(pr: PullRequest) {
		if (sessionStore.getByPrNumber(pr.number)) return;
		const session = sessionStore.addSession(pr);
		if (!session) return;
		// The SessionPanel will handle creating the PTY and running claude
	}

	async function handleAutoFix(pr: PullRequest) {
		const existing = sessionStore.getByPrNumber(pr.number);
		if (existing) {
			// Send /fix-pr command to the existing session's terminal
			await invoke("write_session", { id: existing.id, data: "/fix-pr\r" });
			sessionStore.updateStatus(existing.id, "fixing");
			return;
		}
		const session = sessionStore.addSession(pr);
		if (!session) return;
		sessionStore.updateStatus(session.id, "fixing");
		// The SessionPanel detects "fixing" status and runs auto_fix_pr
	}

	function handleOpenShell() {
		sessionStore.addSession(null);
	}

	function ciStatus(pr: PullRequest): "pass" | "fail" | "pending" | "none" {
		if (!pr.statusCheckRollup || pr.statusCheckRollup.length === 0) return "none";
		const hasFailure = pr.statusCheckRollup.some((c) => c.conclusion === "FAILURE");
		if (hasFailure) return "fail";
		const hasPending = pr.statusCheckRollup.some((c) => c.status === "IN_PROGRESS" || c.status === "QUEUED");
		if (hasPending) return "pending";
		return "pass";
	}
</script>

<aside class="sidebar">
	<div class="sidebar-header">
		<h2>PR Dashboard</h2>
		<button
			class="settings-toggle"
			class:active={showSettings}
			onclick={() => showSettings = !showSettings}
			title="Settings"
		>
			Settings
		</button>
	</div>

	{#if showSettings}
		<div class="settings-panel">
			<label class="settings-field">
				<span class="settings-label">GitHub Token</span>
				<input
					type="password"
					value={settingsStore.githubToken}
					oninput={(e) => settingsStore.setGithubToken(e.currentTarget.value)}
					placeholder="ghp_..."
					class="settings-input"
				/>
			</label>
			<label class="settings-field">
				<span class="settings-label">Claude Credentials Path</span>
				<input
					type="text"
					value={settingsStore.claudeCredentialsPath}
					oninput={(e) => settingsStore.setClaudeCredentialsPath(e.currentTarget.value)}
					placeholder="~/.claude/.credentials.json"
					class="settings-input"
				/>
			</label>
			<label class="settings-field">
				<span class="settings-label">Claude Model</span>
				<select
					value={settingsStore.claudeModel}
					onchange={(e) => settingsStore.setClaudeModel(e.currentTarget.value)}
					class="settings-input"
				>
					<option value="sonnet">Sonnet</option>
					<option value="opus">Opus</option>
					<option value="haiku">Haiku</option>
				</select>
			</label>
			<label class="settings-field">
				<span class="settings-label">Git Name (optional)</span>
				<input
					type="text"
					value={settingsStore.gitUserName}
					oninput={(e) => settingsStore.setGitUserName(e.currentTarget.value)}
					placeholder="Your Name"
					class="settings-input"
				/>
			</label>
			<label class="settings-field">
				<span class="settings-label">Git Email (optional)</span>
				<input
					type="text"
					value={settingsStore.gitUserEmail}
					oninput={(e) => settingsStore.setGitUserEmail(e.currentTarget.value)}
					placeholder="you@example.com"
					class="settings-input"
				/>
			</label>
			<label class="settings-field">
				<span class="settings-label">Docker Template</span>
				<select
					value={settingsStore.dockerTemplate}
					onchange={(e) => settingsStore.setDockerTemplate(e.currentTarget.value)}
					class="settings-input"
				>
					<option value="default">Default</option>
					<option value="mongodb">MongoDB</option>
					<option value="mongodb-replicaset">MongoDB ReplicaSet</option>
				</select>
			</label>
			{#if !settingsStore.isConfigured}
				<div class="settings-warning">GitHub Token and Claude Credentials Path are required.</div>
			{/if}
			<a
				class="usage-link"
				href="https://claude.ai/settings/usage"
				target="_blank"
				rel="noopener noreferrer"
			>View Claude Plan Usage</a>
		</div>
	{/if}

	<form class="repo-form" onsubmit={(e) => { e.preventDefault(); handleRepoSubmit(); }}>
		<input
			type="text"
			bind:value={repoInput}
			placeholder="owner/repo"
			class="repo-input"
		/>
		<button type="submit" class="fetch-btn" disabled={prStore.loading}>
			{#if prStore.loading}
				<span class="spinner"></span>
			{:else}
				Load
			{/if}
		</button>
	</form>

	{#if prStore.error}
		<div class="error-msg">{prStore.error}</div>
	{/if}

	{#if hasLoaded}
		<div class="auto-update-bar">
			<label class="auto-update-label">
				<input type="checkbox" bind:checked={autoUpdate} />
				Auto-refresh
			</label>
			{#if autoUpdate}
				<span class="countdown">{Math.floor(countdown / 60)}:{String(countdown % 60).padStart(2, "0")}</span>
			{/if}
		</div>
	{/if}

	<div class="pr-list">
		{#each prStore.prs as pr (pr.number)}
			{@const status = ciStatus(pr)}
			{@const activeSession = sessionStore.getByPrNumber(pr.number)}
			<div class="pr-item" class:pr-active={!!activeSession} class:pr-just-passed={prStore.recentlyPassed.has(pr.number)}>
				<div class="pr-header">
					<span class="ci-dot ci-{status}" title="CI: {status}"></span>
					<span class="pr-number">#{pr.number}</span>
					<span class="pr-title" title={pr.title}>{pr.title}</span>
				</div>
				<div class="pr-meta">
					<span class="pr-branch" title={pr.headRefName}>{pr.headRefName}</span>
					<span class="pr-author">{pr.author.login}</span>
				</div>
				<div class="pr-actions">
					{#if activeSession}
						<span class="active-badge">
							{activeSession.status === "fixing" ? "Fixing..." : "Active"}
						</span>
						<button
							class="action-btn action-fix"
							onclick={() => handleAutoFix(pr)}
							title="Send /fix-pr to active session"
						>
							Auto Fix
						</button>
					{:else}
						<button
							class="action-btn"
							onclick={() => handleOpenCli(pr)}
							disabled={!sessionStore.canAdd}
							title="Open Claude Code session for this PR"
						>
							Open CLI
						</button>
						<button
							class="action-btn action-fix"
							onclick={() => handleAutoFix(pr)}
							disabled={!sessionStore.canAdd}
							title="Auto-fix: checkout branch, run /fix-pr"
						>
							Auto Fix
						</button>
					{/if}
					<a
						class="action-btn action-link"
						href={pr.url}
						target="_blank"
						rel="noopener noreferrer"
						title="Open PR in browser"
					>
						Open PR
					</a>
				</div>
			</div>
		{/each}

		{#if prStore.prs.length === 0 && !prStore.loading && prStore.repo}
			<div class="empty">No open PRs found</div>
		{/if}

		{#if !prStore.repo}
			<div class="empty">Enter a repository to load PRs</div>
		{/if}
	</div>

	<div class="sidebar-footer">
		<button
			class="shell-btn"
			onclick={handleOpenShell}
			disabled={!sessionStore.canAdd}
		>
			+ New Shell Session
		</button>
		<div class="session-count">{sessionStore.count}/5 sessions</div>
	</div>
</aside>

<style>
	.sidebar {
		width: var(--sidebar-width);
		min-width: var(--sidebar-width);
		height: 100%;
		background: var(--bg-secondary);
		border-right: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.sidebar-header {
		padding: 1rem;
		border-bottom: 1px solid var(--border);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	h2 {
		font-size: 1rem;
		font-weight: 600;
	}

	.settings-toggle {
		padding: 0.2rem 0.5rem;
		font-size: 0.75rem;
		background: var(--bg-tertiary);
		color: var(--text-muted);
		border: 1px solid var(--border);
		border-radius: 4px;
	}

	.settings-toggle:hover,
	.settings-toggle.active {
		color: var(--text-primary);
		border-color: var(--accent);
	}

	.settings-panel {
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.settings-field {
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
	}

	.settings-label {
		font-size: 0.75rem;
		color: var(--text-muted);
	}

	.settings-input {
		padding: 0.35rem 0.5rem;
		background: var(--bg-primary);
		border: 1px solid var(--border);
		border-radius: 4px;
		color: var(--text-primary);
		outline: none;
		font-size: 0.85rem;
	}

	.settings-input:focus {
		border-color: var(--accent);
	}

	select.settings-input {
		appearance: none;
		-webkit-appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23888' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 0.5rem center;
		padding-right: 1.5rem;
		cursor: pointer;
	}

	select.settings-input option {
		background: var(--bg-primary);
		color: var(--text-primary);
	}

	.settings-warning {
		font-size: 0.75rem;
		color: var(--warning);
	}

	.usage-link {
		font-size: 0.75rem;
		color: var(--accent);
		text-decoration: none;
	}

	.usage-link:hover {
		text-decoration: underline;
	}

	.repo-form {
		display: flex;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--border);
	}

	.repo-input {
		flex: 1;
		padding: 0.4rem 0.6rem;
		background: var(--bg-primary);
		border: 1px solid var(--border);
		border-radius: 4px;
		color: var(--text-primary);
		outline: none;
	}

	.repo-input:focus {
		border-color: var(--accent);
	}

	.fetch-btn {
		padding: 0.4rem 0.75rem;
		background: var(--accent);
		color: var(--bg-primary);
		border: none;
		border-radius: 4px;
		font-weight: 600;
	}

	.fetch-btn:hover:not(:disabled) {
		background: var(--accent-hover);
	}

	.fetch-btn:disabled {
		opacity: 0.5;
	}

	.spinner {
		display: inline-block;
		width: 14px;
		height: 14px;
		border: 2px solid var(--bg-primary);
		border-top-color: transparent;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
		vertical-align: middle;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.auto-update-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.4rem 1rem;
		border-bottom: 1px solid var(--border);
		font-size: 0.8rem;
		color: var(--text-muted);
	}

	.auto-update-label {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		cursor: pointer;
	}

	.auto-update-label input {
		cursor: pointer;
	}

	.countdown {
		font-variant-numeric: tabular-nums;
	}

	.error-msg {
		padding: 0.5rem 1rem;
		color: var(--error);
		font-size: 0.85rem;
	}

	.pr-list {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.pr-item {
		padding: 0.75rem;
		border-radius: 6px;
		margin-bottom: 0.25rem;
	}

	.pr-item:hover {
		background: var(--bg-hover);
	}

	.pr-active {
		border-left: 2px solid var(--accent);
	}

	.pr-just-passed {
		animation: highlight-flash 2s ease-out;
	}

	@keyframes highlight-flash {
		0% { background: color-mix(in srgb, var(--success) 40%, transparent); }
		100% { background: transparent; }
	}

	.active-badge {
		font-size: 0.75rem;
		padding: 0.2rem 0.5rem;
		background: var(--accent);
		color: var(--bg-primary);
		border-radius: 4px;
		font-weight: 600;
	}

	.pr-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.ci-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.ci-pass { background: var(--success); }
	.ci-fail { background: var(--error); }
	.ci-pending { background: var(--warning); }
	.ci-none { background: var(--text-muted); }

	.pr-number {
		color: var(--text-muted);
		font-size: 0.85rem;
		flex-shrink: 0;
	}

	.pr-title {
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.pr-meta {
		display: flex;
		gap: 0.75rem;
		font-size: 0.8rem;
		color: var(--text-muted);
		margin-bottom: 0.5rem;
		padding-left: 1rem;
	}

	.pr-branch {
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		max-width: 140px;
	}

	.pr-actions {
		display: flex;
		gap: 0.5rem;
		padding-left: 1rem;
	}

	.action-btn {
		padding: 0.25rem 0.6rem;
		font-size: 0.8rem;
		background: var(--bg-tertiary);
		color: var(--text-secondary);
		border: 1px solid var(--border);
		border-radius: 4px;
	}

	.action-btn:hover:not(:disabled) {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.action-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.action-fix {
		color: var(--accent);
		border-color: var(--accent);
	}

	.action-fix:hover:not(:disabled) {
		background: var(--accent);
		color: var(--bg-primary);
	}

	.action-link {
		text-decoration: none;
		text-align: center;
	}

	.action-link:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.empty {
		padding: 2rem 1rem;
		text-align: center;
		color: var(--text-muted);
		font-size: 0.9rem;
	}

	.sidebar-footer {
		padding: 0.75rem 1rem;
		border-top: 1px solid var(--border);
	}

	.shell-btn {
		width: 100%;
		padding: 0.5rem;
		background: var(--bg-tertiary);
		color: var(--text-secondary);
		border: 1px solid var(--border);
		border-radius: 6px;
		margin-bottom: 0.5rem;
	}

	.shell-btn:hover:not(:disabled) {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.shell-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.session-count {
		text-align: center;
		font-size: 0.8rem;
		color: var(--text-muted);
	}
</style>
