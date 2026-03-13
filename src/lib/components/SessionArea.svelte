<script lang="ts">
	import { sessionStore } from "../stores/sessions.svelte";
	import SessionPanel from "./SessionPanel.svelte";
</script>

<main class="session-area">
	{#if sessionStore.count === 0}
		<div class="empty-state">
			<h2>No active sessions</h2>
			<p>Select a PR from the sidebar to open a Claude Code session,<br />or click "+ New Shell Session" for a plain terminal.</p>
		</div>
	{:else}
		<div class="panels" style="--panel-count: {sessionStore.count}">
			{#each sessionStore.sessions as session (session.id)}
				<SessionPanel {session} />
			{/each}
		</div>
	{/if}
</main>

<style>
	.session-area {
		flex: 1;
		height: 100%;
		overflow: hidden;
		display: flex;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		width: 100%;
		color: var(--text-muted);
		gap: 0.5rem;
	}

	.empty-state h2 {
		font-size: 1.1rem;
		font-weight: 500;
		color: var(--text-secondary);
	}

	.empty-state p {
		font-size: 0.9rem;
		text-align: center;
		line-height: 1.5;
	}

	.panels {
		display: flex;
		width: 100%;
		height: 100%;
		gap: 1px;
		background: var(--border);
	}
</style>
