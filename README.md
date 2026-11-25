# JadroOS

**macOS-inspired look. Windows-inspired power features. Arch Linux underneath. AI built into the desktop.**

JadroOS is an independent, experimental x86-64 desktop distribution. It is not Apple macOS, Microsoft Windows, or an official OpenAI/Anthropic product. Real macOS is the only visual reference; Windows is a reference for behavior and compatibility only.

**Status: Phase 0, Foundation (0.0.1-dev).** No installable image, shell or AI integration exists yet. See [where we are](#where-we-are-now).

## What the finished JadroOS includes

- **macOS-style desktop:** menu bar with global app menus, Dock with magnification, traffic-light windows, glass material, Control Center, Notification Center, Mission Control, lock screen, original icons and wallpapers
- **Windows-style power features:** Alt+Tab through individual windows, snap layouts, task manager, device and service views, per-app audio routing
- **Spotlight with AI:** local search for apps, settings and files, plus tabs for ChatGPT, Claude, Codex, Claude Code and local AI, first as handoffs to the official apps, later with answers and agents built in
- **Safe AI agents:** agents that can do tasks on the PC through a permission broker, with approvals, audit log and a stop button
- **Windows and gaming:** one-click restart into Windows and back, Gaming Center with Steam/Proton, Wine for Windows apps, Windows fallback for anti-cheat titles
- **System apps:** Settings, Files (Finder-style with Quick Look), Activity, Devices, App Center, Welcome, Backup
- **A real distro:** own package repo, live ISO, installer, snapshots before updates with rollback, signed updates, phone sharing

All of it is planned. The [roadmap](docs/planning/roadmap.md) decides the order.

## How it gets built

The desktop comes first, then everything plugs into it. Ten phases ([ADR-0006](docs/decisions/0006-phased-build-order.md)):

| Phase | Result |
|---|---|
| 0 Foundation | Dev environment; empty shell runs nested inside KWin |
| 1 Desktop core | "JadroOS" login session with wallpaper, menu bar, Dock, traffic-light windows |
| 2 Window management | Alt+Tab, snapping, minimize-to-Dock, Mission Control |
| 3 Spotlight + AI handoff | Local search, AI tabs open the official ChatGPT/Claude/Codex/Claude Code |
| 4 Surfaces and polish | Control Center, notifications, lock screen, global menu, glass, themed apps; first real hardware |
| 5 Distro platform | Packages, own repo, live ISO booting in QEMU |
| 6 System apps | Settings, Files, Welcome, Activity, Devices, App Center |
| 7 Native AI and agents | Answers in Spotlight, agents, permission broker, local AI |
| 8 Windows and gaming | Restart into Windows and back, Gaming Center, Wine |
| 9 Installer, updates, release | Installer, rollback, signed updates, backup, phone sharing, public beta |

## Built with

Arch Linux · KWin (Wayland) · Quickshell (QML shell) · Qt 6 / QML apps · C++ for KWin plugins · Rust for services from Phase 3 · archiso · Calamares. Full list with phases: [stack](docs/architecture/stack.md).

## Start here

| You are | Read |
|---|---|
| **The owner** | [Roadmap](docs/planning/roadmap.md) → [Task board](docs/planning/task-board.md) → [VM setup](docs/engineering/vm-setup.md) → [Kickoff prompts](docs/engineering/agent-kickoff-prompts.md) |
| **Claude Code or Codex** | [CLAUDE.md](CLAUDE.md) / [AGENTS.md](AGENTS.md) → [Roadmap](docs/planning/roadmap.md) → [Task board](docs/planning/task-board.md) → [Workflow](docs/engineering/workflow.md) |
| **Curious about the design** | [Requirements](docs/product/requirements.md), [Feature matrix](docs/product/feature-matrix.md), [Desktop design](docs/design/desktop.md), [Spotlight](docs/design/spotlight.md) |

All documents:

| Area | Documents |
|---|---|
| Planning | [Roadmap](docs/planning/roadmap.md) · [Task board](docs/planning/task-board.md) · [Risks](docs/planning/risks.md) |
| Product | [Requirements](docs/product/requirements.md) · [Feature matrix](docs/product/feature-matrix.md) · [System apps](docs/product/system-apps.md) |
| Design | [Desktop](docs/design/desktop.md) · [Spotlight](docs/design/spotlight.md) · [Visual references](docs/design/references.md) |
| Architecture | [Overview](docs/architecture/overview.md) · [Stack](docs/architecture/stack.md) · [AI platform](docs/architecture/ai.md) · [Platform and dual boot](docs/architecture/platform.md) · [Apps and gaming](docs/architecture/compatibility.md) |
| Engineering | [Handbook](docs/engineering/index.md) · [Workflow](docs/engineering/workflow.md) · [Development](docs/engineering/development.md) · [Testing](docs/engineering/testing.md) · [VM setup](docs/engineering/vm-setup.md) · [Contracts](docs/engineering/integration-contracts.md) · [Licensing](docs/engineering/licensing.md) |
| Decisions and security | [ADRs](docs/decisions/README.md) · [Threat model](docs/security/threat-model.md) · [Sources](docs/references.md) |

## Where we are now

Phase 0. Next steps are on the [task board](docs/planning/task-board.md): the owner merges this foundation, picks a license and sets up the dev VM; then Codex builds the nested dev tools and CI, and Claude builds the design tokens and shell skeleton.

What exists today: documentation, an offline Spotlight route-preview prototype in Qt (`desktop/spotlight`, to be replaced in Phase 3), a Rust policy library with tests (`services/policy`, used in Phase 7), a request schema, safe default config and a repo check script.

## Quick start (inside the dev VM, not your main PC)

```sh
cmake -S . -B build -G Ninja
cmake --build build
./build/desktop/spotlight/jadro-spotlight
cargo test --manifest-path services/policy/Cargo.toml
python3 scripts/check_repo.py
```

No admin rights, credentials or paid requests needed. More in the [development guide](docs/engineering/development.md).

## Governance

[Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Agent rules](AGENTS.md) · [Licensing and assets](docs/engineering/licensing.md). No project license has been chosen yet (ticket T0.2); public visibility doesn't grant redistribution rights. Nothing is released before the licensing and security gates pass.
