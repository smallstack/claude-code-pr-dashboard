<script lang="ts">
	import type { AuthStatus } from "../types";

	interface Props {
		authStatus: AuthStatus;
		onRetry: () => void;
	}

	let { authStatus, onRetry }: Props = $props();
</script>

<div class="auth-check">
	<div class="auth-card">
		<h1>Setup Required</h1>
		<p>The following CLI tools need to be authenticated:</p>

		<div class="status-list">
			<div class="status-item" class:ok={authStatus.gh} class:fail={!authStatus.gh}>
				<span class="indicator">{authStatus.gh ? "OK" : "!!"}</span>
				<div>
					<strong>GitHub CLI (gh)</strong>
					{#if !authStatus.gh}
						<p class="hint">Run <code>gh auth login</code> in your terminal</p>
					{/if}
				</div>
			</div>

			<div class="status-item" class:ok={authStatus.claude} class:fail={!authStatus.claude}>
				<span class="indicator">{authStatus.claude ? "OK" : "!!"}</span>
				<div>
					<strong>Claude Code CLI</strong>
					{#if !authStatus.claude}
						<p class="hint">Install Claude Code: <code>npm install -g @anthropic-ai/claude-code</code></p>
					{/if}
				</div>
			</div>
		</div>

		<button class="retry-btn" onclick={onRetry}>Check Again</button>
	</div>
</div>

<style>
	.auth-check {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		padding: 2rem;
	}

	.auth-card {
		background: var(--bg-secondary);
		border: 1px solid var(--border);
		border-radius: 8px;
		padding: 2rem;
		max-width: 480px;
		width: 100%;
	}

	h1 {
		font-size: 1.25rem;
		margin-bottom: 0.5rem;
	}

	p {
		color: var(--text-secondary);
		margin-bottom: 1.5rem;
	}

	.status-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.status-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem;
		border-radius: 6px;
		background: var(--bg-tertiary);
	}

	.indicator {
		font-weight: bold;
		font-size: 0.85rem;
		padding: 2px 8px;
		border-radius: 4px;
		flex-shrink: 0;
	}

	.ok .indicator {
		background: var(--success);
		color: var(--bg-primary);
	}

	.fail .indicator {
		background: var(--error);
		color: var(--bg-primary);
	}

	.hint {
		font-size: 0.85rem;
		color: var(--text-muted);
		margin: 0.25rem 0 0;
	}

	code {
		background: var(--bg-primary);
		padding: 2px 6px;
		border-radius: 3px;
		font-size: 0.85rem;
	}

	.retry-btn {
		width: 100%;
		padding: 0.6rem;
		background: var(--accent);
		color: var(--bg-primary);
		border: none;
		border-radius: 6px;
		font-weight: 600;
	}

	.retry-btn:hover {
		background: var(--accent-hover);
	}
</style>
