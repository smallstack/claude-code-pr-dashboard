import { invoke } from "@tauri-apps/api/core";
import confetti from "canvas-confetti";
import type { AuthStatus, PullRequest } from "../types";

function ciStatusOf(pr: PullRequest): "pass" | "fail" | "pending" | "none" {
	if (!pr.statusCheckRollup || pr.statusCheckRollup.length === 0) return "none";
	if (pr.statusCheckRollup.some((c) => c.conclusion === "FAILURE")) return "fail";
	if (pr.statusCheckRollup.some((c) => c.status === "IN_PROGRESS" || c.status === "QUEUED")) return "pending";
	return "pass";
}

class PrStore {
	prs = $state<PullRequest[]>([]);
	repo = $state<string>(localStorage.getItem("pr-dashboard-repo") ?? "");
	loading = $state<boolean>(false);
	error = $state<string | null>(null);
	authStatus = $state<AuthStatus>({ gh: false, claude: false });

	/** Previous CI status per PR number, for detecting transitions to green */
	private prevCiStatus = new Map<number, string>();

	async checkAuth() {
		try {
			this.authStatus = await invoke<AuthStatus>("check_auth");
		} catch {
			this.authStatus = { gh: false, claude: false };
		}
	}

	async fetchPrs() {
		if (!this.repo) return;

		this.loading = true;
		this.error = null;

		try {
			const newPrs = await invoke<PullRequest[]>("list_prs", { repo: this.repo });

			// Detect PRs that just turned green
			for (const pr of newPrs) {
				const newStatus = ciStatusOf(pr);
				const oldStatus = this.prevCiStatus.get(pr.number);
				if (oldStatus && oldStatus !== "pass" && newStatus === "pass") {
					confetti({
						particleCount: 150,
						spread: 80,
						origin: { y: 0.7 }
					});
					break; // one confetti burst per refresh is enough
				}
			}

			// Update previous status map
			this.prevCiStatus.clear();
			for (const pr of newPrs) {
				this.prevCiStatus.set(pr.number, ciStatusOf(pr));
			}

			this.prs = newPrs;
		} catch (e) {
			this.error = e instanceof Error ? e.message : String(e);
			this.prs = [];
		} finally {
			this.loading = false;
		}
	}

	setRepo(repo: string) {
		this.repo = repo;
		localStorage.setItem("pr-dashboard-repo", repo);
	}
}

export const prStore = new PrStore();
