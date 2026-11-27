# Session (planned, Phase 1)

The "JadroOS" entry at the login screen. Owner: Codex.

| Part | Ticket |
|---|---|
| `jadroos.desktop` (Wayland session file) and `jadro-session` startup script: starts KWin (Wayland + XWayland), Quickshell with `desktop/shell`, XDG portals, polkit agent and the KDE services the session needs, without plasmashell | T1.5 |
| Default user config: `kwinrc` (decoration, effects), global shortcuts, workspace defaults | T1.6, T2.6 |

Rules: installing the session file into `/usr/share/wayland-sessions` is a manual owner step in the dev VM until packaging (T5.1). The stock Plasma session stays installed as a fallback. Per-user defaults are installed without `sudo`.
