import type { PullRequest, Session } from "../types";

const MAX_SESSIONS = 5;
let counter = 0;

function createId(): string {
	counter++;
	return `session-${counter}-${Date.now()}`;
}

class SessionStore {
	sessions = $state<Session[]>([]);

	get count() {
		return this.sessions.length;
	}

	get canAdd() {
		return this.sessions.length < MAX_SESSIONS;
	}

	getByPrNumber(prNumber: number): Session | undefined {
		return this.sessions.find((s) => s.pr?.number === prNumber);
	}

	addSession(pr: PullRequest | null = null): Session | null {
		if (!this.canAdd) return null;

		const session: Session = {
			id: createId(),
			label: pr ? `PR #${pr.number}` : "Shell",
			pr,
			status: "idle"
		};

		this.sessions = [...this.sessions, session];
		return session;
	}

	removeSession(id: string) {
		this.sessions = this.sessions.filter((s) => s.id !== id);
	}

	updateStatus(id: string, status: Session["status"]) {
		this.sessions = this.sessions.map((s) => (s.id === id ? { ...s, status } : s));
	}
}

export const sessionStore = new SessionStore();
