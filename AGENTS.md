# Instructions for coding agents (Codex and Claude Code)

## Read first
1. [Roadmap](docs/planning/roadmap.md): phases, tickets, dependencies, what must wait
2. [Task board](docs/planning/task-board.md): your next ticket
3. [Workflow](docs/engineering/workflow.md): ownership, ticket lifecycle, review levels, definition of done
4. The docs your ticket touches (design, architecture, requirements, ADRs)

## Your role
- **Codex:** platform engineer. Owns `tools/`, `scripts/`, `.github/workflows/`, `platform/`, `services/`, and pure-logic KWin scripts.
- **Claude Code:** desktop engineer. Owns `desktop/` (shell, design system, KWin decoration/effects, Spotlight UI) and app UIs in `apps/`.
- Both: write the code, build it, test it, fix it, show real evidence. Work on one ticket at a time, in your own worktree, on a branch named `<codex|claude>/T<phase>.<n>-<short-name>`.

## How to work
- Only start a ticket whose dependencies are `done`. Respect the "Don't start… before…" table in the roadmap.
- Stay in your paths; anything else needs its own ticket or the owner's OK.
- For visible changes, run the nested session (`tools/dev-nested.sh`) and attach screenshots (`tools/screenshot-nested.sh`).
- Open a PR with the template filled in and update your ticket's row on the task board in the same PR. Never merge.
- Tickets marked "cross-reviews" need the other agent's review before the owner merges.

## Decisions you must keep
Real macOS is the only visual reference (no Apple assets, no KDE lookalike themes as sources) · Alt+Tab switches individual windows in most-recent order · Spotlight always opens on General · ChatGPT Instant goes only to the official app/site · Claude, Codex and Claude Code are separate routes · Arch + KWin + Quickshell shell + Qt apps; Rust only for services from Phase 3 · Windows switch is an explicit reboot.

## Never
- Run `sudo`, use approval-bypass flags, or touch disks, boot entries or firmware. Ask the owner to run privileged steps.
- Put credentials, tokens or API keys in the repo, logs or prompts; scrape cookies; use undocumented provider endpoints.
- Make real AI provider calls or anything that could cost money before Phase 7 and the owner's approval.
- Claim a build, test, screenshot or result you didn't actually produce. "Not run" is fine; faking is not.
- Bypass anti-cheat, grant arbitrary root, or merge/publish releases.
- Rewrite unrelated code or reformat files outside your ticket.
