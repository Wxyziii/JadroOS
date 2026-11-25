# ADR-0001: Upstream foundation

**Status:** accepted. Updated by ADR-0005 (shell host), ADR-0006 (build order) and ADR-0007 (languages).

**Context:** independent Arch-based distro with close macOS visual fidelity, Windows-style functionality and broad PC support.

**Decision:** vanilla Arch; KWin on Wayland as the window manager and compositor, with the stock Plasma session kept as a fallback; a custom QML shell hosted by Quickshell instead of plasmashell (ADR-0005); Qt Quick apps; Rust background services starting in Phase 3 (ADR-0007).

**Alternatives:** CachyOS derivative (faster start, extra upstream coupling); custom compositor (too much risk); theming Plasma (violates ADR-0004); Hyprland (tiling-first, see ADR-0005).

**Consequences:** rolling-release discipline (full upgrades only, tested version pins), dependence on KWin's script/effect/decoration APIs and on Quickshell–KWin compatibility.

**Validation:** T0.5 (window list under KWin), Phase 1–2 exit gates (session, decoration, effects), T4.9 (NVIDIA, multi-monitor, high refresh on real hardware). Revisit only if measured blockers can't be solved upstream.
