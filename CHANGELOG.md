# Changelog

## [0.1.9](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.8...claude-code-pr-dashboard-v0.1.9) (2026-03-23)


### Features

* add shell-only mode and new command to open docker shell ([115df43](https://github.com/smallstack/claude-code-pr-dashboard/commit/115df439b9697e14d4e1d1b3ffebfcd7bbe52c12))


### Bug Fixes

* streamline icon array formatting in tauri configuration ([65993c1](https://github.com/smallstack/claude-code-pr-dashboard/commit/65993c154b8a8e3cb876295bcb9fa59fa770f0d0))

## [0.1.8](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.7...claude-code-pr-dashboard-v0.1.8) (2026-03-15)


### Features

* Add docker compose templates and PR session title ([a1a7784](https://github.com/smallstack/claude-code-pr-dashboard/commit/a1a778426c66fd73763218a6a02ff856aba792c9))


### Bug Fixes

* add proper permissions for temp files ([522a9aa](https://github.com/smallstack/claude-code-pr-dashboard/commit/522a9aa2f126da574186fefe52a00eae4e953d55))
* linter issues ([731b0c5](https://github.com/smallstack/claude-code-pr-dashboard/commit/731b0c52625b0e55b1f4d72ecae83e6b00ac8495))
* Use 1:1 git configs ([152c125](https://github.com/smallstack/claude-code-pr-dashboard/commit/152c125361194cd5d7a587c2c6336c77a5dab03a))

## [0.1.7](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.6...claude-code-pr-dashboard-v0.1.7) (2026-03-14)


### Features

* add formatting step for release-please changes in workflow ([64f13a2](https://github.com/smallstack/claude-code-pr-dashboard/commit/64f13a2b3e56478f28bbd4bf7f46a880c4178e70))


### Bug Fixes

* streamline icon array formatting in tauri configuration ([75cbf38](https://github.com/smallstack/claude-code-pr-dashboard/commit/75cbf38efeb058f4b733c1b2505bef6b3da18ff7))

## [0.1.6](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.5...claude-code-pr-dashboard-v0.1.6) (2026-03-14)


### Features

* add CLAUDE.md for project documentation and update package scripts ([1a34df3](https://github.com/smallstack/claude-code-pr-dashboard/commit/1a34df3822fe2e4d6de837e438d9c2c4dd578789))


### Bug Fixes

* add Volta configuration for Node.js version 24.14.0 ([5c4af23](https://github.com/smallstack/claude-code-pr-dashboard/commit/5c4af238196060e8fc20df6850c1770a2177e8dc))

## [0.1.5](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.4...claude-code-pr-dashboard-v0.1.5) (2026-03-14)


### Bug Fixes

* embed docker files in binary instead of using compile-time path ([77bcecf](https://github.com/smallstack/claude-code-pr-dashboard/commit/77bcecfacb207ae570344cfb3fbddb7ffe47666c))

## [0.1.4](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.3...claude-code-pr-dashboard-v0.1.4) (2026-03-14)


### Bug Fixes

* sync tauri and cargo version with package.json for correct release artifacts ([bcbb138](https://github.com/smallstack/claude-code-pr-dashboard/commit/bcbb138bb4a57d7431e74f37ced3167d5f1269a1))

## [0.1.3](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.2...claude-code-pr-dashboard-v0.1.3) (2026-03-13)


### Bug Fixes

* replace confetti animation with PR/panel highlight flash ([55e0a25](https://github.com/smallstack/claude-code-pr-dashboard/commit/55e0a25f987db05887c3b2a39c29e5594791dbd5))

## [0.1.2](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.1...claude-code-pr-dashboard-v0.1.2) (2026-03-13)


### Bug Fixes

* update schema path and add bundle configuration in tauri.conf.json ([fd77ff8](https://github.com/smallstack/claude-code-pr-dashboard/commit/fd77ff87446c440af91bba13ad9c8036eaf97fae))

## [0.1.1](https://github.com/smallstack/claude-code-pr-dashboard/compare/claude-code-pr-dashboard-v0.1.0...claude-code-pr-dashboard-v0.1.1) (2026-03-13)


### Features

* add release-please configuration for automated releases ([f1322ab](https://github.com/smallstack/claude-code-pr-dashboard/commit/f1322ab24e0bb8dd2521e6c0c17533d7a940cfa7))
* enhance SessionArea.svelte with background styling and improved empty state message ([f1322ab](https://github.com/smallstack/claude-code-pr-dashboard/commit/f1322ab24e0bb8dd2521e6c0c17533d7a940cfa7))
* refactor session management and enhance GitHub integration ([a4f50f4](https://github.com/smallstack/claude-code-pr-dashboard/commit/a4f50f43ec7d68a10ab8ad2e9a02752fb8f70140))
* scaffold Tauri v2 + Svelte 5 PR dashboard app ([78336de](https://github.com/smallstack/claude-code-pr-dashboard/commit/78336de1d73cb1b4ff1d447a6d75463382fd5dc8))


### Bug Fixes

* format confetti origin in prs.svelte.ts for consistency ([f1322ab](https://github.com/smallstack/claude-code-pr-dashboard/commit/f1322ab24e0bb8dd2521e6c0c17533d7a940cfa7))
* reorder import statement in main.ts for clarity ([f1322ab](https://github.com/smallstack/claude-code-pr-dashboard/commit/f1322ab24e0bb8dd2521e6c0c17533d7a940cfa7))
* reorder imports in App.svelte for consistency ([f1322ab](https://github.com/smallstack/claude-code-pr-dashboard/commit/f1322ab24e0bb8dd2521e6c0c17533d7a940cfa7))
* use void operator for fetchPrs in Sidebar.svelte to avoid unhandled promise ([f1322ab](https://github.com/smallstack/claude-code-pr-dashboard/commit/f1322ab24e0bb8dd2521e6c0c17533d7a940cfa7))
