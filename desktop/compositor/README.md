# Compositor (KWin) integration

Code that runs inside or configures KWin. Owner: Claude, except KWin scripts that are pure window logic (Codex, e.g. T2.2).

| Part | Language | Ticket | Phase |
|---|---|---|---|
| `decoration/`: traffic-light window decoration (KDecoration plugin) | C++ | T1.4 | 1 |
| `tabbox/`: Alt+Tab task switcher theme (individual windows, MRU, previews) | QML | T2.1 | 2 |
| `scripts/snap/`: snap layouts, keyboard snapping, group restore | JS (KWin script) | T2.2 | 2 |
| Snap-layout popup on the green button | QML | T2.3 | 2 |
| `effects/minimize/`: minimize-to-Dock animation | QML/JS effect | T2.4 | 2 |
| `effects/overview/`: Mission Control | QML effect | T2.5 | 2 |
| `effects/glass/`: blur/tint material for shell surfaces, solid fallback | KWin effect | T4.6 | 4 |

Use KWin's public script/effect/decoration APIs; no fork of KWin. Pin and test against specific KWin versions. Never sample the lock screen or protected content.
