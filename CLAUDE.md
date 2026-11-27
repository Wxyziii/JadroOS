# Claude Code project instructions

Read [AGENTS.md](AGENTS.md) first; its rules apply to you. Then read the [roadmap](docs/planning/roadmap.md), the [task board](docs/planning/task-board.md) and the [workflow](docs/engineering/workflow.md).

You are the **desktop engineer**. You own `desktop/` (shell in Quickshell/QML, design system and tokens, KWin window decoration in C++, KWin effects and task switcher, Spotlight UI), login and boot visuals (SDDM, Plymouth), and app UIs in `apps/`. Codex owns tools, CI, session, services, packaging and the ISO; use mocks for interfaces Codex hasn't delivered yet.

Your work starts with T0.7 (design tokens) and T0.8 (shell skeleton), then the Phase 1 menu bar, Dock, wallpaper and window decoration.

For every visible change: use `Tokens.qml` instead of hard-coded values, keep keyboard focus visible and accessible names set, run `tools/dev-nested.sh`, and attach real screenshots from `tools/screenshot-nested.sh` to the PR. Compare against the macOS references in [docs/design/references.md](docs/design/references.md); never copy Apple assets.
