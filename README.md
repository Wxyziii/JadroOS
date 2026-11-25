# JadroOS

**macOS-inspired experience. Windows-inspired functionality. Arch Linux underneath. AI built into the desktop.**

JadroOS is an independent, experimental x86-64 desktop distribution project. It is not Apple macOS, Microsoft Windows, or an official OpenAI/Anthropic product. Actual Apple macOS is the sole visual reference; Windows is a behavior and compatibility reference only.

**Status: architecture and starter scaffold (0.0.1-dev).** No installable JadroOS image, functioning compositor integration, or production agent system exists yet. The desktop prototype and policy library are deliberately non-privileged.

## Start here

| Document | Purpose |
|---|---|
| [Vision and requirements](docs/product/requirements.md) | Product goals, confirmed decisions, acceptance criteria |
| [Feature comparison](docs/product/feature-matrix.md) | macOS vs Windows vs JadroOS feature decisions |
| [Architecture](docs/architecture/overview.md) | Services, dependencies, trust boundaries and data flow |
| [Technology stack](docs/architecture/stack.md) | Implementation choices and alternatives |
| [Visual reference gallery](docs/design/references.md) | Real Apple images, not Linux themes |
| [Desktop design](docs/design/desktop.md) | Interaction model, glass material, accessibility |
| [Spotlight and providers](docs/design/spotlight.md) | Search, tabs, model routing, keyboard behavior |
| [AI platform](docs/architecture/ai.md) | Providers, authentication, usage and tool execution |
| [Security](docs/security/threat-model.md) | Permissions, trust, threat mitigations |
| [OS and dual boot](docs/architecture/platform.md) | Arch, updates, storage, firmware |
| [Apps and gaming](docs/architecture/compatibility.md) | Compatibility tiers and Windows fallback |
| [System applications](docs/product/system-apps.md) | Settings, file manager, developer and gaming centers |
| [Roadmap and backlog](docs/planning/roadmap.md) | Milestones, exit gates and dependencies |
| [Testing](docs/engineering/testing.md) | Verification strategy and hardware matrix |
| [Development](docs/engineering/development.md) | Build instructions, CI and project conventions |
| [Decisions](docs/decisions/README.md) | Architecture decisions and unresolved items |
| [Sources](docs/references.md) | Official technical references |
| [Risk register](docs/planning/risks.md) | Key engineering and product risks |

## Parallel development

Both Codex and Claude Code implement and validate their own work, then independently cross-review. Begin with the [engineering handbook](docs/engineering/index.md), [VM setup](docs/engineering/vm-setup.md), [kickoff prompts](docs/engineering/agent-kickoff-prompts.md) and [parallel task board](docs/planning/task-board.md). CI implementation is assigned to the two initial coding tickets; no automated gates are enabled yet.

## Repository map

- `desktop/spotlight/`: Qt Quick prototype, no provider network calls.
- `services/policy/`: Rust authorization and routing reference library.
- `services/{agent,search,system}/`: boundary specifications.
- `protocols/`: versioned request schema.
- `config/`: safe defaults, not installed automatically.
- `platform/`: image, packaging and recovery scaffolding.
- `tests/`: manual acceptance specifications.
- `scripts/`: local structural checks.

## Quick start (Linux development machine)

See [development](docs/engineering/development.md). Do not install this on your primary system. First use a VM. The prototype uses Qt 6 and CMake; the policy crate uses stable Rust.

```sh
cmake -S . -B build
cmake --build build
./build/desktop/spotlight/jadro-spotlight
cargo test --manifest-path services/policy/Cargo.toml
python3 scripts/check_repo.py
```

No administrator access, provider credentials or paid requests are needed for these initial checks.

## Governance

See [CONTRIBUTING](CONTRIBUTING.md), [SECURITY](SECURITY.md), [AGENTS](AGENTS.md), and [asset/license policy](docs/engineering/licensing.md). The owner has not selected a project license yet; no third-party asset rights are implied. Do not ship a public binary before the licensing and security gates are met.
