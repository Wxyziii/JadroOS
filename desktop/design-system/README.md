# Design system

The single source of truth for how JadroOS looks. Owner: Claude.

| Part | Ticket | Phase |
|---|---|---|
| `tokens.json`: colors (light/dark, high contrast), radii, spacing, typography, shadows, motion curves and durations | T0.7 | 0 |
| Generated `Tokens.qml` singleton used by shell, KWin QML and apps | T0.7 | 0 |
| Theme generator: tokens to Kvantum (Qt apps), GTK3 theme, GTK4 color CSS | T4.7 (Codex) | 4 |
| Original icons, cursor, sounds | T4.8 | 4 |
| Shared app components (sidebar, toolbar, lists, forms, sheets) | T6.1 | 6 |

Real macOS is the visual reference ([references](../../docs/design/references.md), [desktop design](../../docs/design/desktop.md)). Only original or properly licensed assets; no Apple fonts, icons or wallpapers; no third-party macOS-lookalike KDE themes as sources.
