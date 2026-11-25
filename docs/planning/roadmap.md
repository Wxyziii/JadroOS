# Roadmap
Milestones are dependency-based, not delivery promises. One-person project should optimize for a usable vertical slice. Avoid building all apps before proving desktop/session and AI boundaries.

| Phase | Deliverables | Exit gate |
|---|---|---|
| 0 Foundation | docs, repo, Qt prototype, policy tests, licenses decision | checks pass; architecture reviewed |
| 1 Developer preview | Arch VM, KWin session, menu/Dock skeleton, Alt+Tab, General Spotlight, official chat/CLI handoff | keyboard and accessibility tests; no root; session fallback works |
| 2 Desktop alpha | shell effects, Snap, workspaces, Files, Settings, local indexing, native device views | stable multi-monitor; UI review; safe file ops |
| 3 Platform alpha | signed packages, live ISO, snapshot recovery, boot switch spike, Steam/Wine | two-disk VM safety; rollback; physical GPU/game tests |
| 4 Agent beta | SIWC eligibility spike, Codex adapter, permission broker, tool execution and audit | security review, adversarial tests, stop/revoke, no quota surprises |
| 5 Public beta | installer, signed update channel, device sharing, compatibility matrix, accessibility | recovery and installer tests, legal review, documented hardware limits |
| 6 Stable | release lifecycle, support, regression suite, third-party licenses/SBOM | no critical defects; rollback and support policy |

## Initial backlog
| Ticket | Priority | Depends | Definition of done |
|---|---|---|---|
| J-001 | P0 | none | Select code license; document contributor terms |
| J-002 | P0 | none | Qt/CMake build on Arch, screenshot actual prototype, keyboard test |
| J-003 | P0 | J-002 | KWin Wayland MRU Alt+Tab and reverse/cancel; no shell regression |
| J-004 | P0 | J-002 | General app index parses .desktop safely, tests ranking/cancel |
| J-005 | P0 | J-002 | Official ChatGPT/Claude app handoff and explicit copy fallback |
| J-006 | P0 | J-005 | Provider capability registry and usage warning, negative tests |
| J-007 | P0 | J-003 | KWin effect prototype and frame profiling; fallback tint |
| J-008 | P0 | J-006 | Auth eligibility and provider terms verified, no secrets logged |
| J-009 | P0 | none | Threat model review, typed broker interface and negative tests |
| J-010 | P1 | J-003 | Snap layouts and window group restore |
| J-011 | P1 | J-004 | Files vertical slice with Quick Look and safe transfer |
| J-012 | P1 | J-009 | D-Bus/polkit mock then constrained system operation |
| J-013 | P1 | J-001 | Clean package build, signed local repo, VM live image |
| J-014 | P1 | J-013 | Two-disk UEFI boot-switch and rollback test |
| J-015 | P1 | J-013 | Steam/Proton physical NVIDIA smoke test |
| J-016 | P1 | J-008,J-009 | Codex app-server stream/cancel and constrained agent task |
| J-017 | P1 | J-012,J-016 | Portal desktop capture/input consent and revoke |
| J-018 | P2 | J-016 | Local AI and bounded scheduled jobs |
| J-019 | P2 | J-013 | Installer safety and public beta checklist |
| J-020 | P2 | J-011 | Paired device transfer/clipboard, iOS limits |

## Working cadence
Small PRs, one issue and acceptance test per slice, weekly integration VM smoke test, architecture decision records for changes. Track burn-down of exit gates rather than invented dates. First next task is J-002, then J-003/J-004/J-005. Do not test firmware/disk operations on daily PC before VM gates.
