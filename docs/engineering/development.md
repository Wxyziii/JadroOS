# Development guide

How the code is organized and how to build and run it. For setting up the machine, see [vm-setup.md](vm-setup.md). For the process (branches, PRs, reviews), see [workflow.md](workflow.md).

## The main dev loop: nested session

Almost all desktop work in Phases 0–3 happens in a **nested session**: KWin runs inside a normal window in the dev VM, with the JadroOS shell loaded in it. Changing the Dock and seeing the result takes seconds, with no reboot, ISO or login.

```sh
tools/dev-nested.sh              # start KWin in a window with desktop/shell loaded (T0.4)
tools/screenshot-nested.sh out.png   # save what the nested session shows (T0.4)
```

Agents must use the screenshot script to check UI changes and attach the PNG to their PR. From Phase 1 the same shell also runs as a full login session ("JadroOS" at the login screen).

These scripts are created in ticket T0.4. Until then, the only runnable parts are the old prototype and checks below.

## Repository layout

```
desktop/
  design-system/   tokens.json, generated Tokens.qml, shared QML components   (Claude)
  shell/           Quickshell config: menu bar, Dock, Spotlight, Control Center… (Claude)
  compositor/      KWin decoration (C++), effects, task switcher, scripts      (Claude; logic scripts Codex)
  spotlight/       early Qt prototype, kept building until Phase 3 replaces it  (Claude)
apps/              Settings, Files, Welcome, Activity… (Phase 6)                (Claude UI, Codex backends)
services/
  policy/          Rust authorization rules (reference library, used in Phase 7)
  search/          search service (Rust, Phase 3)
  agent/           AI agent service and providers (Rust, Phase 7)
  system/          privileged broker (Rust, Phase 7)
platform/
  session/         JadroOS login session (Phase 1)
  packages/        PKGBUILDs (Phase 5)
  archiso/         live ISO profile (Phase 5)
  recovery/        snapshots/recovery (Phase 9)
  windows-switch/  restart-into-Windows helpers (Phase 8)
protocols/         versioned contracts (JSON schema, D-Bus XML)
config/            safe default settings
tools/             dev and VM scripts (Phase 0+)
scripts/           repo checks used by CI
tests/             manual acceptance checklists
docs/              everything you're reading
```

Folders whose README says "planned" contain no code yet. They get code when their phase starts.

## Build and check (today)

```sh
cmake -S . -B build -G Ninja
cmake --build build
./build/desktop/spotlight/jadro-spotlight      # old offline prototype window
cargo test --manifest-path services/policy/Cargo.toml
python3 scripts/check_repo.py
```

No admin rights, credentials or paid requests are needed. The Spotlight prototype is a normal window, not the real Spotlight. It only shows which route a question would take.

## Languages

See [ADR-0007](../decisions/0007-language-policy.md).

| What | Language |
|---|---|
| All UI: shell, apps, KWin effects and task switcher | QML / JavaScript |
| KWin window decoration, Qt/QML plugins, small helpers | C++20 |
| Background services (search, agent, broker) | Rust, from Phase 3 |
| Dev, CI and build scripts | Bash, Python |

Don't add another language without a new ADR.

## Conventions

- **QML:** every interactive item has an accessible name and visible focus; colors, sizes and timings come from `Tokens.qml`, never hard-coded.
- **C++:** C++20, RAII, no raw `new` without an owner.
- **Rust:** stable toolchain, no `unsafe` without review, typed errors.
- **Everywhere:** no shell-string concatenation for commands, no embedded credentials, structured logs without personal data. New dependencies need a one-line license and maintenance note in the PR.

## Packaging and releases (Phase 5+)

Packages are built in clean chroots from reviewed sources with pinned revisions and checksums. The package database and artifacts are signed. Each release records toolchain, versions, hashes, licenses/SBOM and build date. Channels: dev, alpha, stable. Nothing is published before the license is chosen and the release checklist passes.
