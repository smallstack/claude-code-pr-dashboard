<script lang="ts">
	import { onMount } from "svelte";
	import AuthCheck from "./lib/components/AuthCheck.svelte";
	import Sidebar from "./lib/components/Sidebar.svelte";
	import SessionArea from "./lib/components/SessionArea.svelte";
	import { prStore } from "./lib/stores/prs.svelte";

	let authChecked = $state(false);
	let authOk = $state(false);

	onMount(async () => {
		await prStore.checkAuth();
		authOk = prStore.authStatus.gh && prStore.authStatus.claude;
		authChecked = true;
	});
</script>

{#if !authChecked}
	<div class="loading">Checking authentication...</div>
{:else if !authOk}
	<AuthCheck authStatus={prStore.authStatus} onRetry={async () => {
		await prStore.checkAuth();
		authOk = prStore.authStatus.gh && prStore.authStatus.claude;
	}} />
{:else}
	<div class="app-layout">
		<Sidebar />
		<SessionArea />
	</div>
{/if}

<style>
	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-muted);
		font-size: 14px;
	}

	.app-layout {
		display: flex;
		height: 100%;
		width: 100%;
		position: relative;
		z-index: 1;
	}
</style>
