export interface PullRequest {
	number: number;
	title: string;
	headRefName: string;
	state: string;
	author: {
		login: string;
	};
	url: string;
	updatedAt: string;
	statusCheckRollup: CheckStatus[];
}

export interface CheckStatus {
	name?: string;
	context?: string;
	status?: string;
	conclusion?: string | null;
}

export interface Session {
	id: string;
	label: string;
	pr: PullRequest | null;
	status: "idle" | "running" | "fixing";
}

export interface AuthStatus {
	gh: boolean;
	claude: boolean;
}
