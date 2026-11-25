# ADR-0007: Language policy

**Status:** accepted

## Context
Every extra language adds toolchains, CI jobs, review effort and context for the agents. The original stack used C++/QML for UI and Rust for services from the start, although no service is needed before Phase 3.

## Decision
| Area | Language | Starts |
|---|---|---|
| Shell, apps, KWin effects and task switcher | QML / JavaScript | Phase 0 |
| KWin window decoration, Qt/QML plugins, small integration helpers | C++20 | Phase 1 |
| Background services: search (Phase 3), agent and providers (Phase 7), system broker (Phase 7), boot switch helper (Phase 8) | Rust (stable) | Phase 3 |
| Dev, CI and build scripts | Bash, Python | Phase 0 |

The existing `services/policy` Rust crate stays as a tested reference library and is wired into the broker in Phase 7. A new language needs a new ADR.

## Alternatives considered
- **C++ for services too:** fewer languages, but Rust's memory safety matters for privileged and network-facing services.
- **Rust from Phase 0:** no service exists to write yet; it would only add CI and review load.

## Consequences
Phases 0–2 are QML/C++ only (plus scripts). Codex's Phase 0–2 work is tooling, CI, session and KWin scripts; Rust work starts with T3.2.
