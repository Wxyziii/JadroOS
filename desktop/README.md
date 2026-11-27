# Desktop

Everything the user sees: the shell, the window decoration and effects, the design system. Owner: Claude ([workflow.md](../docs/engineering/workflow.md)).

| Folder | What goes here | Phase |
|---|---|---|
| [design-system/](design-system/README.md) | `tokens.json`, generated `Tokens.qml`, shared QML components | 0 |
| [shell/](shell/README.md) | Quickshell config: menu bar, Dock, wallpaper, Spotlight UI, Control Center, notifications | 0–4 |
| [compositor/](compositor/README.md) | KWin decoration (C++), effects, task switcher theme, scripts | 1–4 |
| `spotlight/` | Early offline Qt prototype. Kept building until the shell's Spotlight replaces it in T3.3, then removed | — |

Real macOS ([references](../docs/design/references.md)) is the only visual reference. The stock widgets in the prototype are not design targets.
