class SettingsStore {
	githubToken = $state(localStorage.getItem("pr-dashboard-github-token") ?? "");
	claudeCredentialsPath = $state(localStorage.getItem("pr-dashboard-claude-creds") ?? "~/.claude/.credentials.json");
	claudeModel = $state(localStorage.getItem("pr-dashboard-claude-model") ?? "sonnet");
	gitUserName = $state(localStorage.getItem("pr-dashboard-git-name") ?? "");
	gitUserEmail = $state(localStorage.getItem("pr-dashboard-git-email") ?? "");

	get isConfigured(): boolean {
		return this.githubToken.length > 0 && this.claudeCredentialsPath.length > 0;
	}

	setGithubToken(token: string) {
		this.githubToken = token;
		localStorage.setItem("pr-dashboard-github-token", token);
	}

	setClaudeCredentialsPath(path: string) {
		this.claudeCredentialsPath = path;
		localStorage.setItem("pr-dashboard-claude-creds", path);
	}

	setClaudeModel(model: string) {
		this.claudeModel = model;
		localStorage.setItem("pr-dashboard-claude-model", model);
	}

	setGitUserName(name: string) {
		this.gitUserName = name;
		localStorage.setItem("pr-dashboard-git-name", name);
	}

	setGitUserEmail(email: string) {
		this.gitUserEmail = email;
		localStorage.setItem("pr-dashboard-git-email", email);
	}
}

export const settingsStore = new SettingsStore();
