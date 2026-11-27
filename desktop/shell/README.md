# Shell

The JadroOS desktop shell, written in QML and run by **Quickshell** on KWin ([ADR-0005](../../docs/decisions/0005-quickshell-shell.md)). Owner: Claude.

Run it during development with `tools/dev-nested.sh` (KWin in a window) and check it with `tools/screenshot-nested.sh`.

## Planned contents and when
| Part | Ticket | Phase |
|---|---|---|
| Skeleton (entry point, empty top/bottom bars using tokens) | T0.8 | 0 |
| Wallpaper layer | T1.1 | 1 |
| Menu bar v1 (focused app, fallback app menu, clock, status) | T1.2 | 1 |
| Dock v1 (pinned, running, launch/focus, magnification) | T1.3 | 1 |
| Spotlight overlay (General + AI tabs) | T3.3, T3.5, T3.6 | 3 |
| Control Center | T4.1 | 4 |
| Notifications and Notification Center | T4.2 | 4 |
| Global menu in the menu bar | T4.5 | 4 |
| Remote session indicator with Disconnect | T6.8 | 6 |

## Rules
- All colors, sizes, radii and timings come from `Tokens.qml` (design system), never hard-coded.
- Every interactive item has an accessible name and visible keyboard focus; reduced motion and transparency respected.
- How the shell gets the window list from KWin is decided by spike T0.5 (see ADR-0005).
- The shell never reads lock-screen or other secure surfaces and never injects input.
