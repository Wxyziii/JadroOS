# ADR-0005: Quickshell hosts the desktop shell

**Status:** accepted (window-list approach pending T0.5 spike)

## Context
The menu bar, Dock, Spotlight overlay, Control Center and notifications must be Wayland **layer-shell** surfaces: anchored to screen edges, reserving space, staying above windows. A normal Qt window can't do this reliably. Plasma's own shell (plasmashell) is not used (ADR-0001, ADR-0004), so JadroOS needs its own shell host.

## Decision
The shell is written in QML and hosted by **Quickshell**, running on KWin (Wayland). Quickshell provides layer-shell windows, per-monitor instancing, a notification server, system tray, PipeWire/MPRIS/UPower integration, a session lock surface and live reload, all usable from QML. Shell code lives in `desktop/shell/`.

- Spotlight's UI is part of the shell (an overlay), talking to the search service over D-Bus.
- Apps (Settings, Files…) are **not** Quickshell; they are normal Qt Quick applications in `apps/`, sharing the design-system components.
- KWin handles windows, effects and decorations (`desktop/compositor/`).

## Open question for the T0.5 spike: window list and focus
The Dock, menu bar and later Alt+Tab-adjacent features need: list of windows, focused window, app id, activate, minimize, close. Quickshell's built-in toplevel support targets Wayland protocols that KWin may not implement. Options, in order of preference:

1. **Import KDE's task manager QML module** (`org.kde.taskmanager`, shipped with plasma-workspace) inside Quickshell. This is the same model Plasma's task bar uses (grouping, activation, minimize, MRU). Needs plasma-workspace installed (not plasmashell running).
2. **Wayland protocol supported by both**, if the installed KWin version implements one Quickshell can use.
3. **KWin script publishing window state over D-Bus**, plus a small Quickshell-side adapter.

The spike records: what was tried, KWin and Quickshell versions, what works (list, focus changes, activate, minimize, per-monitor), and the decision. Update this ADR with a "Spike result" section.

## Fallback
If none of the options work well enough, the shell host changes to **our own Qt/C++ application using LayerShellQt** plus KDE's task manager library. The QML components stay the same; only the host process and some services change. This fallback would be a new ADR.

## Alternatives considered
- **Theming plasmashell:** rejected; the Plasma look and layout keep showing through (ADR-0004).
- **Own C++ shell from day one:** valid, but weeks of plumbing that Quickshell already provides. Kept as fallback.
- **Hyprland + Quickshell:** tiling-first window management fights the macOS floating-window model; KWin's snapping, effects and NVIDIA support are stronger for this project.

## Consequences
Dependency on Quickshell's release cadence and its compatibility with KWin; versions are pinned and tested together. Global menu needs a separate helper (T4.4) because Quickshell doesn't read KWin's app-menu information directly.
