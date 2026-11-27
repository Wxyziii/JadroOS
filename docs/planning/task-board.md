# Task board

What is being worked on right now, and what each person picks up next. Ticket details (deliverable, dependencies) are in the [roadmap](roadmap.md); this page only tracks status.

**How to update this page:** the PR that starts or finishes a ticket also changes its row here. The owner corrects it when merging if needed. GitHub PRs are the source of truth if this page and a PR disagree.

Statuses: `todo` · `in progress` · `in review` · `done` · `blocked (reason)`

## Current phase: 0, Foundation

### Up next
| Who | Next ticket(s) | Notes |
|---|---|---|
| Owner | T0.1 merge foundation, T0.2 license, T0.3 dev VM | Agents can't start until T0.1 and T0.3 are done |
| Codex | T0.4 → T0.5 (spike), T0.6 in parallel | T0.5 decides how the shell talks to KWin. Claude's Phase 1 dock/menu bar depend on it |
| Claude | T0.7 → T0.8 | T0.8 needs Codex's `dev-nested.sh` (T0.4) |

## Status

### Phase 0: Foundation
| Ticket | Owner | Status | PR |
|---|---|---|---|
| T0.1 Merge foundation | Owner | in review | PR #1 |
| T0.2 License | Owner | todo | |
| T0.3 Dev VM + agents | Owner | todo | |
| T0.4 `dev-nested.sh` + screenshot script | Codex | todo | |
| T0.5 Spike: window list under KWin | Codex | todo | |
| T0.6 CI | Codex | todo | |
| T0.7 Design tokens | Claude | todo | |
| T0.8 Shell skeleton | Claude | todo | |

### Phase 1: Desktop core
| Ticket | Owner | Status | PR |
|---|---|---|---|
| T1.1 Wallpaper | Claude | todo | |
| T1.2 Menu bar v1 | Claude | todo | |
| T1.3 Dock v1 | Claude | todo | |
| T1.4 Window decoration | Claude | todo | |
| T1.5 JadroOS session | Codex | todo | |
| T1.6 Session defaults | Codex | todo | |

### Phase 2: Window management
| Ticket | Owner | Status | PR |
|---|---|---|---|
| T2.1 Alt+Tab switcher | Claude | todo | |
| T2.2 Snap KWin script | Codex | todo | |
| T2.3 Snap layout popup | Claude | todo | |
| T2.4 Minimize-to-Dock | Claude | todo | |
| T2.5 Mission Control | Claude | todo | |
| T2.6 Workspace defaults + regression checklist | Codex | todo | |

### Phase 3: Spotlight + AI handoff
| Ticket | Owner | Status | PR |
|---|---|---|---|
| T3.1 Search contract | Codex | todo | |
| T3.2 Search service (Rust) | Codex | todo | |
| T3.3 Spotlight UI | Claude | todo | |
| T3.4 Provider handoff adapter | Codex | todo | |
| T3.5 AI tabs UI | Claude | todo | |
| T3.6 Calculator / units | Claude | todo | |
| T3.7 Search integration | Codex | todo | |

### Phases 4–9
Not started. Rows are added here when the phase before them passes its exit gate. Until then, the ticket list lives only in the [roadmap](roadmap.md).

## Workload balance
Claude has more tickets in Phases 1–2 (visual work); Codex has more in Phases 3, 5 and 7. When one agent is idle waiting on a dependency, the owner can give it a ticket from the next phase whose dependencies are met, or have it cross-review the other agent's open PR.
