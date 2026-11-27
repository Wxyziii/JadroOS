# Risk register
Severity/likelihood are preliminary qualitative ratings; reassess at each phase exit gate.

| Risk | Severity | Likelihood | Mitigation / gate |
|---|---|---|---|
| KWin private ABI breakage | high | high | supported API first, pin tested versions, fallback session |
| Quickshell–KWin integration gaps (window list, focus, global menu) | high | medium | T0.5 spike before Phase 1 UI work; fallback to own Qt shell with LayerShellQt (ADR-0005) |
| VM can't show real GPU behavior | medium | high | real-hardware testing from Phase 4 (T4.9) instead of at the end |
| True glass compositor performance | high | high | effect spike, frame budget, static tint fallback |
| Provider auth/terms change | high | high | adapter isolation, official docs, no unauthorized workarounds |
| Instant quota misrouting | high | medium | explicit routing tests and no silent fallback |
| Agent prompt injection/root abuse | critical | high | separate broker, approvals, sandbox, negative tests |
| Remote access abuse or exposure | critical | medium | off by default, tailnet-only ports with test, second factor, portal consent, indicator, cross-review (T6.7) |
| Unattended remote access unsupported on Wayland/KWin | medium | medium | attended mode first; unattended only via a supported, revocable portal grant (T6.7) |
| Dual boot firmware/disk damage | critical | medium | separate disk, VM, readback, backups |
| Rolling upgrade breaks GPU | high | medium | full upgrades, LTS, snapshot plus ESP plan |
| Anti-cheat/app incompatibility | high | high | compatibility matrix, Windows fallback |
| Apple IP/branding rights | high | medium | original assets, reference links, review before release |
| iOS closed protocols | medium | high | scoped open pairing, avoid parity promises |
| Scope exceeds maintainer capacity | high | high | phased roadmap (ADR-0006), at most two phases in flight, exit gates, reuse upstream |
| Supply-chain compromise | critical | medium | signed packages, clean builds, SBOM, review |
| Accessibility regressions | high | medium | keyboard/AT-SPI and contrast gates |
| Public repo secrets | high | medium | no credentials, scanning and revoke process |

No risk is accepted automatically by this document. Owner reviews each release gate; critical open risks block public distribution.
