# How work gets done

This page is the whole development process: who does what, how a ticket goes from idea to merged, when a second review is needed, and what "done" means. It replaces the older separate documents (agent coordination, code ownership, quality gates, definition of done, cross-review, bug management).

## Roles

| Role | Does | Never does |
|---|---|---|
| **Owner** (the human) | Picks priorities, reviews and merges every PR, runs anything that needs `sudo`, tests on real hardware, approves licenses, security-sensitive changes and releases | — |
| **Claude Code** | Builds the **desktop**: shell (menu bar, Dock, Spotlight UI, Control Center…), KWin decoration and effects, design system, themes, app UIs | Run `sudo`, merge PRs, edit Codex-owned paths without a ticket |
| **Codex** | Builds the **platform**: dev/VM tooling, CI, session, search and agent services, providers, KWin scripts that are pure logic, packaging, ISO, installer, boot switch | Run `sudo`, merge PRs, edit Claude-owned paths without a ticket |

Both agents are full engineers: they write code, build it, test it, fix it, and attach real evidence. Neither is only a reviewer.

## Who owns which folder

| Path | Author | Notes |
|---|---|---|
| `desktop/shell/`, `desktop/spotlight/`, `desktop/design-system/` | Claude | |
| `desktop/compositor/` (decoration, effects, task switcher) | Claude | KWin *scripts* that are pure logic (e.g. T2.2 snapping) are Codex |
| `apps/` | Claude (UI) + Codex (backends) | Each ticket lists exact paths |
| `services/` | Codex | |
| `platform/` (session, packages, archiso, recovery, windows-switch) | Codex | Login/boot visuals (SDDM, Plymouth) are Claude |
| `tools/`, `scripts/`, `.github/workflows/` | Codex | |
| `protocols/`, `config/defaults.json` | Contract tickets only | Cross-review required |
| Root `CMakeLists.txt` | Whoever the ticket names | Conflict hotspot: one ticket at a time |
| `docs/` | Ticket author for their area | Roadmap/task board changes: owner approves |

A ticket may name other exact paths. Anything outside your paths needs its own ticket or the owner's OK.

## The life of a ticket

1. **Pick** the next ticket for you from the [task board](../planning/task-board.md). Check its dependencies in the [roadmap](../planning/roadmap.md) are `done`.
2. **Branch** from the latest `main` in your own worktree: `claude/T1.3-dock` or `codex/T0.4-dev-nested`.
3. **Build it**: smallest complete version that meets the deliverable. Add tests. Update docs touched by the change.
4. **Check it yourself**: build, run tests and lint, and for anything visible take screenshots with `tools/screenshot-nested.sh` (Phase 0+) or the VM screenshot tool (Phase 5+). Fix what fails and rerun.
5. **Open a PR** with the template filled in: what changed, exact commands run and their results, screenshots, what you could *not* test. Set the ticket to `in review` on the task board in the same PR.
6. **Review** (see next section).
7. **Owner merges.** Ticket goes to `done`. If `main` breaks after a merge, the owner reverts or opens a fix ticket. Tests are never disabled to make something pass.

## Review levels

| Level | Used for | Who reviews |
|---|---|---|
| **Standard** | Most tickets: UI, effects, tooling, docs, app features | Owner reviews the PR (CI must be green). The owner can ask the other agent for a review whenever they want a second opinion |
| **Cross-review required** | Shared contracts (`protocols/`, D-Bus interfaces), anything privileged or security-related (lock screen, broker, polkit), AI providers/auth/billing, boot/disk/installer/update code, CI and release workflows | The *other* agent reviews the exact commit in a separate checkout, runs the checks, tries at least one failure case, and replies **APPROVE**, **REQUEST CHANGES** or **BLOCKED** with file:line details. Then the owner reviews and merges |

Tickets that need cross-review are marked "(X cross-reviews)" in the roadmap. If fixes are pushed, the reviewer rechecks the new commit; an approval only covers the commit it was given for.

Cross-review checkout, from the clean main checkout:
```sh
git fetch origin
git worktree add --detach ../review origin/codex/T3.1-search-contract
# read, build and test in ../review — don't run unknown scripts blindly
git worktree remove ../review
```

## Definition of done

A ticket is **done** when:
- [ ] The deliverable in the roadmap exists and works
- [ ] Tests added for new behavior, including at least one failure/edge case; existing tests still pass
- [ ] Build, lint and CI are green
- [ ] Visible changes have real screenshots; keyboard focus and light/dark checked
- [ ] Docs updated where behavior or interfaces changed
- [ ] No secrets, no Apple assets, no privileged shortcuts
- [ ] Required review completed; owner merged
- [ ] Task board updated

"Ready for review" is not "done". A skipped or impossible check is written down as **not run**, never as passed.

## Bugs

File a GitHub issue with the bug template: expected vs. actual, steps, commit, environment, severity.

| Severity | Meaning | Effect |
|---|---|---|
| Critical | Data loss, security hole, won't boot | Blocks every merge in that area until fixed |
| High | Core feature broken (Dock won't launch apps, Spotlight crashes) | Blocks the phase exit gate |
| Medium | Works but wrong in some cases | Fix in the same phase or the owner defers it explicitly |
| Low | Polish | Tracked, fixed when convenient |

Fix flow: reproduce, write a failing test if possible, fix, test, PR. If you can't reproduce, write down what you tried instead of closing it as fixed.

## Safety rules (always apply)

- Agents never run `sudo`, use approval-bypass flags, or touch disks, boot entries or firmware. The owner does privileged steps by hand.
- No credentials, tokens or API keys in the repo, logs, prompts or screenshots.
- No real AI provider calls or sign-in flows before Phase 7; no billing-affecting behavior without the owner's approval.
- No Apple fonts, icons, wallpapers or other assets. Real macOS is the visual reference only ([ADR-0004](../decisions/0004-visual-reference.md)).
- Never claim a test ran or passed if it didn't.
- Agents use separate worktrees and never edit each other's branches.
