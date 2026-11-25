# Risk register
Severity/likelihood are preliminary qualitative ratings; reassess after spikes.

| Risk | Severity | Likelihood | Mitigation / gate |
|---|---|---|---|
| KWin private ABI breakage | high | high | supported API first, pin tested versions, fallback session |
| True glass compositor performance | high | high | effect spike, frame budget, static tint fallback |
| Provider auth/terms change | high | high | adapter isolation, official docs, no unauthorized workarounds |
| Instant quota misrouting | high | medium | explicit routing tests and no silent fallback |
| Agent prompt injection/root abuse | critical | high | separate broker, approvals, sandbox, negative tests |
| Dual boot firmware/disk damage | critical | medium | separate disk, VM, readback, backups |
| Rolling upgrade breaks GPU | high | medium | full upgrades, LTS, snapshot plus ESP plan |
| Anti-cheat/app incompatibility | high | high | compatibility matrix, Windows fallback |
| Apple IP/branding rights | high | medium | original assets, reference links, review before release |
| iOS closed protocols | medium | high | scoped open pairing, avoid parity promises |
| Scope exceeds maintainer capacity | high | high | vertical slices, prioritize, reuse upstream |
| Supply-chain compromise | critical | medium | signed packages, clean builds, SBOM, review |
| Accessibility regressions | high | medium | keyboard/AT-SPI and contrast gates |
| Public repo secrets | high | medium | no credentials, scanning and revoke process |

No risk is accepted automatically by this document. Owner reviews each release gate; critical open risks block public distribution.
