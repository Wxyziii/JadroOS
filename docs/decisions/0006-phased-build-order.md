# ADR-0006: Build in phases, desktop first

**Status:** accepted

## Context
The product scope is large and all of it is wanted in the final OS: macOS-style desktop, Windows-style window management, Spotlight with AI, native AI agents, Windows reboot switch, gaming, app compatibility, system apps, installer, updates and phone sharing. Building all areas in parallel would produce many half-finished parts and no usable desktop to test them in. Most features depend on the shell, the session or the design system existing first.

## Decision
Follow the ten phases in the [roadmap](../planning/roadmap.md):
0 Foundation → 1 Desktop core → 2 Window management → 3 Spotlight + AI handoff → 4 System surfaces and polish (first real hardware) → 5 Distro platform → 6 System apps → 7 Native AI and agents → 8 Windows and gaming → 9 Installer, updates and release.

- **No feature is dropped.** The order only decides when each is built.
- At most two phases in flight; each phase has an exit gate with real evidence.
- Safety-sensitive work (privileged code, provider credentials, disks/firmware) has fixed earliest phases.
- Spotlight with AI tabs is moved early (Phase 3) because it's a core part of the product; its first version hands questions to the official apps and sites, and native answers arrive in Phase 7.

## Alternatives considered
- **Platform first (ISO, packaging, installer before the desktop):** produces a bootable image with nothing distinctive on it. Rejected.
- **AI platform first:** needs a desktop to live in and the hardest security work. Rejected as a starting point.
- **Everything in parallel:** too much integration risk for one owner and two agents.

## Consequences
The previous J-xxx backlog is replaced by phase tickets (T<phase>.<n>). Rust services, ISO work and real-hardware testing start later than in the original plan. The desktop is testable from Phase 1 onward, and the real PC is used from Phase 4.
