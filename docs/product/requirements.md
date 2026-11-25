# Product requirements — draft baseline v0.1
Date: 2026-11-25. Status: proposed unless explicitly marked Confirmed.

## Product contract
Confirmed: Arch-based foundation; actual macOS is the exclusive visual reference (no KDE lookalike theme as a design source); Windows functionality and gaming compatibility; Alt+Tab switches individual windows; Spotlight opens on General and includes AI provider tabs; use official provider apps where possible; regular ChatGPT should be accessible independently of Codex; minimize consumption of advanced usage by using official Instant chat for ordinary questions; one-click reboot into Windows and a reverse path.

Baseline: vanilla Arch, KWin/Wayland, a Quickshell-hosted QML shell (ADR-0005), Qt Quick apps, Rust services from Phase 3 (ADR-0007), UEFI x86-64, first validate in VM, then separate SSD. No need to fork a kernel or write a compositor before validating upstream integration. Build order: [ADR-0006](../decisions/0006-phased-build-order.md).

## Requirements (IDs are permanent)
| ID | Phase | Requirement and acceptance |
|---|---|---|
| UX-01 | 1 (global menu 4) | Original Jadro shell with top menu, Dock, window controls, light/dark materials; compare against actual macOS reference set; no third-party theme dependency. |
| UX-02 | 2 | Alt+Tab MRU order, individual windows, reverse with Shift, Escape cancels, no keyboard grab leakage; excludes secure surfaces. |
| UX-03 | 3 | Spotlight General every fresh invocation; searches apps/settings/files locally; keyboard-only usable; results deterministic for exact matches. |
| AI-01 | 3 | ChatGPT tab defaults Instant; opens official website/native client if supported; never silently substitutes quota-consuming inference. |
| AI-02 | 3 handoff, 7 native | Claude, Codex and Claude Code routes independently; supported official app/CLI or explicit fallback; no credential scraping. |
| AI-03 | 7 | Medium/High/Extra High options only when provider advertises capability; prompt, mode and usage warning visible. |
| AI-04 | 7 | Native agent execution with plan, permission scopes, preview, audit, cancel; high-risk actions require local user approval. |
| AI-05 | 7 | Local models, scheduled jobs, multi-agent cooperation with bounded budgets, no privilege escalation via delegation. |
| OS-01 | 5 (live ISO), 9 (installer) | UEFI VM boots Arch baseline; installer later must never touch unselected disk; recovery media available. |
| OS-02 | 8 | Discover valid Windows EFI target, show confirmation, write one-shot target, reboot; reverse helper separately tested. |
| OS-03 | 9 | Signed packages, full upgrades, pre-upgrade snapshots, verified rollback; separate backups. |
| APP-01 | 6 (native/Flatpak/AppImage), 8 (Wine) | Unified launcher with native, Flatpak, AppImage and per-app Wine prefix; unsupported formats explained. |
| GAME-01 | 8 | Steam and Proton, controller/audio, NVIDIA/AMD/Intel validation, high refresh and VRR gates; no anti-cheat bypass. |
| FS-01 | 6 | Finder-like navigation, Quick Look, tags, transfer queue, conflict handling, safe mount handling. |
| SYS-01 | 4 (Control Center), 6 (apps) | Settings, process/device/service/log managers, audio routing, monitor settings and network tools. |
| CON-01 | 9 | Cross-platform transfer/clipboard with explicit pairing; iOS restrictions displayed; no claim of full Continuity parity. |
| SEC-01 | every phase; broker in 7 | No permanent agent root, credentials in keyring, no public listener, no secrets in logs, explicit approval and revocation. |
| A11Y-01 | every phase from 1 | Keyboard navigation, focus visibility, accessible names, reduced motion/transparency, high contrast and scaling. |
| REL-01 | 1 (Plasma fallback), 9 (offline recovery, signing) | Session crash recovers to usable fallback; updates can be recovered offline; release artifacts signed. |

Phase numbers refer to the [roadmap](../planning/roadmap.md). Every requirement is part of the final OS; the phase only says when it is built.

## Non-goals and limits
No claim to execute every executable, emulate proprietary macOS APIs, mirror all iPhone functions, support all anti-cheat, or duplicate Apple artwork/fonts. No cross-OS live process migration; switching is a reboot. No silent paid fallback or unrestricted always-on root agents. No guarantee of official AI app integration on Linux before supported interfaces are verified.

## Personas and workflows
Daily desktop user: launch apps, manage windows/files, ask Instant questions. Gamer: see tested compatibility, launch Linux games or explicitly reboot to Windows. Developer/IT administrator: inspect hardware, services and logs, use SSH/containers, delegate a scoped task and review the diff. Offline user: normal shell/search/file manager and optional local AI continue without cloud.

## Success metrics (targets, not measured)
On reference hardware: warm Spotlight first paint p95 <100 ms, local app results p95 <150 ms; UI frame time within refresh budget when possible, with adaptive glass fallback; idle shell memory target <500 MiB excluding compositor, browser and AI; zero unintended disk writes during installation tests; zero unauthorized privileged actions in adversarial tests. Record environment and sample size; do not promise 400 Hz glass at all loads.
