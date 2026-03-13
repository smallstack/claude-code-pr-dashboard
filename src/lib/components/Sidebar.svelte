<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { prStore } from "../stores/prs.svelte";
	import { sessionStore } from "../stores/sessions.svelte";
	import type { PullRequest } from "../types";

	let repoInput = $state(prStore.repo);

	async function handleRepoSubmit() {
		prStore.setRepo(repoInput.trim());
		await prStore.fetchPrs();
	}

	function handleOpenCli(pr: PullRequest) {
		const session = sessionStore.addSession(pr);
		if (!session) return;
		// The SessionPanel will handle creating the PTY and running claude
	}

	async function handleAutoFix(pr: PullRequest) {
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
	</div>

	<form class="repo-form" onsubmit={(e) => { e.preventDefault(); handleRepoSubmit(); }}>
		<input
			type="text"
			bind:value={repoInput}
			placeholder="owner/repo"
			class="repo-input"
		/>
		<button type="submit" class="fetch-btn" disabled={prStore.loading}>
			{prStore.loading ? "..." : "Load"}
		</button>
	</form>

	{#if prStore.error}
		<div class="error-msg">{prStore.error}</div>
	{/if}

	<div class="pr-list">
		{#each prStore.prs as pr (pr.number)}
			{@const status = ciStatus(pr)}
			<div class="pr-item">
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
	}

	h2 {
		font-size: 1rem;
		font-weight: 600;
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
