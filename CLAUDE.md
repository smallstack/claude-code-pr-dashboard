# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Tauri v2 desktop app for managing multiple GitHub PRs with parallel Claude Code sessions running in Docker containers. Frontend is Svelte 5 (runes), backend is Rust, terminals use xterm.js + portable-pty.

## Commands

```bash
npm install              # Install frontend dependencies
npm run dev              # Vite dev server (port 1420, frontend only)
npm run build            # Build frontend
npm run check            # TypeScript + Svelte type checking (svelte-check)
npm run lint             # Biome lint check
npm run lint:fix         # Biome auto-fix
cargo tauri dev          # Run the full desktop app (builds frontend + Rust)
npm run docker:rebuild   # Rebuild Docker image with --no-cache
```

CI runs: `npm ci` → `npm run lint` → `npm run check` → `npm run build` (Node 22).

## Architecture

**Tauri v2**: Frontend (Svelte/TS in `/src`) communicates with Rust backend (`/src-tauri/src`) via `invoke()` IPC calls. Rust backend shells out to `gh` CLI for GitHub data and manages Docker containers via `docker compose`.

**Key backend files** (`src-tauri/src/`):
- `commands.rs` — Tauri command handlers (`#[tauri::command]`), entry points for all frontend→backend calls
- `pty.rs` — PTY/Docker session lifecycle: spawns containers, manages terminal I/O via Tauri events (`pty-output-{sessionId}`)
- `github.rs` — GitHub CLI wrapper for PR listing and auth checks

**Key frontend files** (`src/lib/`):
- `stores/` — Svelte 5 class-based stores: `prs.svelte.ts` (PR list/status), `sessions.svelte.ts` (active terminal sessions, max 5), `settings.svelte.ts` (persisted to localStorage)
- `components/` — `Sidebar.svelte` (PR list + settings), `SessionArea.svelte` (terminal grid), `SessionPanel.svelte` (xterm.js terminal), `AuthCheck.svelte`
- `types.ts` — TypeScript interfaces (PullRequest, Session, AuthStatus, CheckStatus)

**Docker** (`docker/`): Each Claude Code session runs in an isolated container (Node 24-bookworm base) that clones the repo, checks out the PR branch, and runs Claude Code with `--dangerously-skip-permissions`.

## Coding Patterns

- **Svelte 5 runes**: Use `$state()`, `$derived()`, `$effect()`, `$props()` — not legacy `$:` reactive declarations
- **Rust commands**: Return `Result<T, String>` for error propagation to frontend
- **Biome config**: Tab indentation, 120-char line width, formatter disabled for `.svelte` files
- **Styling**: CSS custom properties for dark theme (Tokyonight-inspired), scoped `<style>` blocks

## Commit Convention

Format: `type(scope): subject` (conventional commits via commitlint + lefthook pre-commit hooks)

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

Rules: lowercase scope, no uppercase start on subject, no trailing period.
