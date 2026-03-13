import { existsSync } from "node:fs";
import { join } from "node:path";

const missing = ["GITHUB_TOKEN"].filter((key) => !process.env[key]);

const home = process.env.HOME || process.env.USERPROFILE;
const credPath = process.env.CLAUDE_CREDENTIALS || join(home, ".claude", ".credentials.json");
if (!existsSync(credPath)) {
	console.error(
		`\n  Claude credentials not found at: ${credPath}\n  Run 'claude' once to authenticate, or set CLAUDE_CREDENTIALS to the path.\n`
	);
	process.exit(1);
}

if (missing.length > 0) {
	console.error(
		`\n  Missing required environment variables:\n${missing.map((k) => `    - ${k}`).join("\n")}\n\n  Export them before running:\n    export GITHUB_TOKEN="ghp_..."\n`
	);
	process.exit(1);
}

console.log("  Preflight checks passed.");
