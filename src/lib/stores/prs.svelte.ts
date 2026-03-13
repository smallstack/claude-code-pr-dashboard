import { invoke } from "@tauri-apps/api/core";
import type { AuthStatus, PullRequest } from "../types";

class PrStore {
	prs = $state<PullRequest[]>([]);
	repo = $state<string>("");
	loading = $state<boolean>(false);
	error = $state<string | null>(null);
	authStatus = $state<AuthStatus>({ gh: false, claude: false });

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
			this.prs = await invoke<PullRequest[]>("list_prs", { repo: this.repo });
		} catch (e) {
			this.error = e instanceof Error ? e.message : String(e);
			this.prs = [];
		} finally {
			this.loading = false;
		}
	}

	setRepo(repo: string) {
		this.repo = repo;
	}
}

export const prStore = new PrStore();
