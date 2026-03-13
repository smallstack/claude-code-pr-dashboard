# Claude Code PR Dashboard

A Tauri v2 desktop app for managing GitHub PRs with integrated Claude Code sessions. Browse PRs in a native sidebar, spawn up to 5 side-by-side terminal panels running Claude Code, and auto-fix PRs with a single click.

## Architecture

```
┌──────────────────────────────────────────────────────┐
│  ┌──────────┐  ┌──────────────────────────────────┐  │
│  │ Sidebar  │  │ Terminal Sessions (1-5)          │  │
│  │          │  │                                  │  │
│  │ owner/   │  │ ┌──────────┐ ┌──────────┐       │  │
│  │ repo     │  │ │ PR #42   │ │ Shell    │  ...  │  │
│  │ [Load]   │  │ │ claude>  │ │ $        │       │  │
│  │          │  │ └──────────┘ └──────────┘       │  │
│  │ PR List  │  │                                  │  │
│  │ #42 ...  │  │                                  │  │
│  │ #38 ...  │  │                                  │  │
│  └──────────┘  └──────────────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

## Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri v2 CLI](https://v2.tauri.app/start/prerequisites/)
- [GitHub CLI](https://cli.github.com/) (`gh`) — authenticated via `gh auth login`
- [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) — authenticated (uses your Claude subscription)
- System dependencies for Tauri (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

## Setup

```bash
# Install dependencies
npm install

# Install Tauri CLI
cargo install tauri-cli --version "^2"

# Run in development mode
cargo tauri dev
```

## Authentication

This app uses **zero custom auth** — it relies entirely on existing CLI authentication:

- **GitHub**: Run `gh auth login` once. The app calls `gh` under the hood.
- **Claude Code**: Run `claude` once to authenticate with your Claude subscription. No API keys needed.

On first launch, the app checks both tools are authenticated and guides you if not.

## Usage

1. Enter a GitHub repository (e.g. `owner/repo`) in the sidebar
2. Click **Load** to fetch open PRs
3. For each PR you can:
   - **Open CLI** — opens a Claude Code session in a terminal panel
   - **Auto Fix** — checks out the branch and runs `/fix-pr` automatically
4. Click **+ New Shell Session** for a plain terminal
5. Up to 5 sessions can run side-by-side

## Tech Stack

- **Tauri v2** — lightweight desktop shell
- **Svelte 5** — reactive frontend with runes
- **xterm.js** — terminal emulation
- **portable-pty** (Rust) — PTY management for Claude Code sessions
- **gh CLI** — GitHub PR data
- **claude CLI** — AI-powered PR fixing
