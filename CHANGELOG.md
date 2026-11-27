# Changelog
## Unreleased (planning update)
- Add built-in remote control over Tailscale: RDP remote desktop and Remote app in Phase 6 (T6.7, T6.8), Sunshine/Moonlight game streaming in Phase 8 (T8.7); requirements REM-01 to REM-03, design in docs/architecture/remote-access.md, off-by-default config.
- Replace the J-xxx backlog with a ten-phase roadmap (T<phase>.<n> tickets, exit gates, "what must wait" rules); desktop first, all planned features kept (ADR-0006).
- Shell hosted by Quickshell on KWin, with a window-list spike before Phase 1 (ADR-0005); Rust only for services from Phase 3 (ADR-0007).
- Merge seven process documents into one workflow guide with two review levels; new task board with per-agent next steps.
- Update VM setup (Quickshell, KWin dev packages, nested session), kickoff prompts, testing (per-phase environments, CI plan), stack, folder READMEs and acceptance checklists by phase.

## 0.0.1-dev — 2026-11-25
- Establish product requirements, design references, architecture, security and roadmap.
- Add offline Qt Quick Spotlight route-preview scaffold and pure Rust routing/policy tests.
- Add safe default configuration, draft protocol and platform directory contracts.
- No installable ISO, actual AI provider connection, compositor integration or privileged service.
