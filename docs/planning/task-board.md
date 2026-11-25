# Initial parallel task board
State at documentation creation: all implementation/validation tickets below NOT STARTED. Foundation documentation exists on PR #1; no build/test results are implied. GitHub PR/issues become operational source of truth once created. This board is a plan, not a live status feed.

| Ticket | Author | Dependencies | Deliverable / exit gate |
|---|---|---|---|
| J-002A | Codex | accepted foundation | Rust tests, structural checker, backend CI, evidence, cross-review |
| J-002B | Claude | accepted foundation | Qt build, UI tests, desktop CI, evidence, cross-review |
| J-002I | designated integrator | J-002A/B approved | combined build, tests, smoke, owner merge |
| J-004A | Codex | J-002I | search contract, introspection and fixtures approved by both |
| J-003 | Codex | J-002I | KWin individual-window MRU Alt+Tab spike, tests and fallback |
| J-021 | Claude | J-002I | shared QML tokens/components, visual reference review and accessibility tests |
| J-004B | Codex | J-004A | safe app index/search, ranking and cancellation tests |
| J-022 | Claude | J-004A,J-021 | Spotlight UI with mock search contract, keyboard/visual tests |
| J-023 | designated integrator | J-004B,J-022 | real search integration and end-to-end checks |
| J-005A | Codex | J-004A | provider handoff adapter and routing failure tests |
| J-005B | Claude | J-021 | provider UI/explicit copy fallback, no silent paid route |
| J-005I | designated integrator | J-005A/B | offline and official handoff manual checks |

Original roadmap J-002 decomposed into A/B/I; J-004 and J-005 likewise. J-001 license owner decision can proceed independently; resolve before public redistribution and before outside contributions. J-007 compositor glass spike after J-003; phase 2+ follows roadmap. Do not start privileged broker or ISO installation before safety gates.

## Claim protocol
Owner assigns one active implementation ticket per agent. Ticket records branch, base SHA, exclusive paths, acceptance, dependencies. PR status: assigned -> implementing -> self-validated -> review -> changes requested/approved -> integration -> merged -> regression verified. Blocked check remains blocked. Both agents may work concurrently on independent tickets; do not start dependent work against changing contracts.
