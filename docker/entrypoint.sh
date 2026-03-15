#!/bin/bash
set -e

# Required: GITHUB_TOKEN, REPO (owner/repo format)
# Optional: BRANCH (checkout a specific branch), GIT_USER_NAME, GIT_USER_EMAIL

if [ -z "$REPO" ]; then
	echo "Error: REPO environment variable is required (e.g. owner/repo)"
	exit 1
fi

# gh CLI uses GH_TOKEN natively
if [ -n "$GITHUB_TOKEN" ]; then
	export GH_TOKEN="$GITHUB_TOKEN"
fi

# Copy host auth credentials (single file mount, may be owned by different uid)
mkdir -p /home/claude/.claude
if [ -f "/home/claude/.claude-host-credentials.json" ]; then
	sudo cp /home/claude/.claude-host-credentials.json /home/claude/.claude/.credentials.json
	sudo chown claude:claude /home/claude/.claude/.credentials.json
fi

# Use gh as git credential helper so fetch/pull/push work with GITHUB_TOKEN
gh auth setup-git

# Configure git user (use env vars or sensible defaults)
git config --global user.email "${GIT_USER_EMAIL}"
git config --global user.name "${GIT_USER_NAME}"

# Clone the repository
REPO_DIR="/home/claude/workspace"
echo "--- Cloning $REPO ---"
gh repo clone "$REPO" "$REPO_DIR"
cd "$REPO_DIR"

# Install dependencies if package.json exists
if [ -f "package.json" ]; then
	echo "--- Installing dependencies ---"
	npm ci
fi

# Checkout branch if specified
if [ -n "$BRANCH" ]; then
	echo "--- Checking out $BRANCH ---"
	git checkout "$BRANCH"
	git pull
fi

# Inject bundled commands (skip if repo already has its own version)
mkdir -p "$REPO_DIR/.claude/commands"
for cmd in /usr/local/share/claude-commands/*.md; do
	name=$(basename "$cmd")
	if [ ! -f "$REPO_DIR/.claude/commands/$name" ]; then
		cp "$cmd" "$REPO_DIR/.claude/commands/$name"
	fi
done

# Trust the workspace and accept bypass-permissions
CONFIG_FILE="/home/claude/.claude.json"
node -e "
	const fs = require('fs');
	let cfg = {};
	try { cfg = JSON.parse(fs.readFileSync('$CONFIG_FILE', 'utf8')); } catch {}
	cfg.hasCompletedOnboarding = true;
	cfg.theme = cfg.theme || 'dark';
	cfg.bypassPermissionsModeAccepted = true;
	cfg.projects = cfg.projects || {};
	cfg.projects['$REPO_DIR'] = { hasTrustDialogAccepted: true };
	fs.writeFileSync('$CONFIG_FILE', JSON.stringify(cfg));
"

# Launch claude with dangerously-skip-permissions (safe inside container)
MODEL_FLAG=""
if [ -n "$CLAUDE_MODEL" ]; then
	MODEL_FLAG="--model $CLAUDE_MODEL"
fi

exec claude --dangerously-skip-permissions $MODEL_FLAG "$@"
