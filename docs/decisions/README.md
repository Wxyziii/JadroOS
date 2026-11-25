# Architecture decision records

| ADR | Status | Decision |
|---|---|---|
| [0001](0001-foundation.md) | accepted | Vanilla Arch + KWin (Wayland) + Qt/QML, Plasma as fallback session |
| [0002](0002-provider-routing.md) | confirmed UX / proposed backend | ChatGPT Instant goes to the official app/site; Claude, Codex, Claude Code are separate routes |
| [0003](0003-privilege.md) | proposed security baseline | Typed broker and least privilege; no permanent root for agents |
| [0004](0004-visual-reference.md) | confirmed | Real macOS is the only visual reference |
| [0005](0005-quickshell-shell.md) | accepted (spike pending) | Quickshell hosts the shell; window-list approach decided by T0.5 |
| [0006](0006-phased-build-order.md) | accepted | Ten phases, desktop first, nothing dropped |
| [0007](0007-language-policy.md) | accepted | QML for UI, C++ for KWin/Qt plugins, Rust for services from Phase 3 |

## Still open
| Decision | Needed by |
|---|---|
| Project license | T0.2 (Phase 0) |
| UI font (Inter or another open font) | T0.7 (Phase 0) |
| Window-list integration approach | T0.5 (Phase 0) |
| Lock screen: themed upstream locker vs. custom | T4.3 (Phase 4) |
| Packaging/signing setup | T5.2 (Phase 5) |
| Bootloader and snapshot layout | T9.2 (Phase 9) |
| Target device list, localization, release cadence | Before public beta (Phase 9) |

New ADR format: status, context, decision, alternatives, consequences, validation.
