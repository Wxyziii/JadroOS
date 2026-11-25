# Roadmap: how and when JadroOS gets built

This is the **master plan**. Every other planning document follows it. If something here disagrees with another document, this file wins (and the other one should be fixed).

**Everything in the product docs is in scope for the final OS.** Spotlight with AI, native AI agents, the Windows reboot switch, Gaming Center, app compatibility, Files, Settings, the installer, phone sharing: all of it ships. The roadmap only decides **the order**. Features are built on top of the layers they depend on, so the desktop comes first and everything else plugs into it.

Phases are dependency-based, not dates.

---

## The phases at a glance

| Phase | Name | What you can do when it's finished | Main owner |
|---|---|---|---|
| **0** | Foundation | Build the repo and see an empty JadroOS shell running in a window inside the dev VM | Owner, both agents |
| **1** | Desktop core | Log into a "JadroOS" session with wallpaper, menu bar, Dock and traffic-light windows | Claude (UI), Codex (session) |
| **2** | Window management | Alt+Tab, snapping, minimize-to-Dock, Mission Control and workspaces | Claude, Codex |
| **3** | Spotlight + AI handoff | Spotlight finds apps, settings and files, and sends questions to ChatGPT/Claude/Codex/Claude Code | Both |
| **4** | System surfaces and polish | Control Center, notifications, lock screen, global menu, glass, themed apps. **First tests on real hardware** | Claude, Codex, Owner |
| **5** | Distro platform | Packages, your own pacman repo, a live ISO that boots into JadroOS in QEMU | Codex, Claude (login/boot visuals) |
| **6** | System apps | Settings, Files, Welcome, Activity, Devices and App Center | Claude (UI), Codex (backends) |
| **7** | Native AI and agents | Answers inside Spotlight, Codex/Claude agents, permission broker, local AI | Codex (services), Claude (UI) |
| **8** | Windows and gaming | One-click restart into Windows (and back), Gaming Center, Wine/Proton app support | Codex, Claude, Owner (hardware) |
| **9** | Installer, updates and release | Installer, snapshots and rollback, signed updates, backup, phone sharing, public beta, then stable | Both, Owner |

```mermaid
flowchart LR
  P0[0 Foundation] --> P1[1 Desktop core] --> P2[2 Window mgmt] --> P3[3 Spotlight + AI handoff] --> P4[4 Surfaces + polish]
  P4 --> P5[5 Distro platform] --> P6[6 System apps] --> P7[7 Native AI + agents] --> P8[8 Windows + gaming] --> P9[9 Installer, updates, release]
```

## Rules for "when"

1. **At most two phases in flight.** You may start a ticket from the next phase once all of its dependencies are done. You may not start a phase until the phase two steps before it has passed its exit gate. Example: Phase 3 tickets can start while Phase 2 is finishing, but Phase 4 waits for Phase 2's exit gate.
2. **Exit gates are real.** A phase is finished when every item in its exit gate is checked off with real evidence (screenshots, test output). "It should work" is not evidence.
3. **Things that must wait** (these are safety rules, not preferences):

| Don't start... | ...before | Why |
|---|---|---|
| Rust background services | Phase 3 (search service is the first) | No service needs to exist before then; see [ADR-0007](../decisions/0007-language-policy.md) |
| Any root/privileged code (polkit helpers, system broker) | Phase 7 | Needs the permission broker design and security review first |
| Real AI provider API calls or sign-in flows | Phase 7 | Phase 3 only opens the official apps/sites. No credentials, no billing |
| ISO / archiso work | Phase 5 | Nothing worth shipping on an ISO until the desktop exists |
| Testing on real hardware | Phase 4 | Glass, animations, NVIDIA and high refresh rates can't be judged in a VM |
| Anything touching disks, boot entries or firmware | Phase 8 (boot switch), Phase 9 (installer), and **only in VMs** until the VM gate passes | Can destroy data or the Windows install |
| Your own versions of mature apps (mail, office, browser) | After Phase 9 | Use existing apps first |

4. **The desktop must always stay usable.** The Plasma session remains installed as a fallback in every phase until Phase 9 makes rollback reliable.

---

## Phase 0: Foundation

**Goal:** the repo is merged, the dev VM works, and a "hello world" shell runs nested inside KWin in a window.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T0.1 | Owner | none | Review and merge the foundation PR into `main` |
| T0.2 | Owner | none | Choose a license ([licensing.md](../engineering/licensing.md); GPL-3.0-or-later recommended for shell/KWin parts) |
| T0.3 | Owner | T0.1 | Dev VM set up per [vm-setup.md](../engineering/vm-setup.md), with both agents installed and worktrees created |
| T0.4 | Codex | T0.1 | `tools/dev-nested.sh`: starts `kwin_wayland --windowed` with `desktop/shell` loaded in Quickshell; `tools/screenshot-nested.sh` saves a PNG of the nested session (agents use this to check their work) |
| T0.5 | Codex | T0.4 | **Spike:** how the Quickshell shell gets the window list, focused window, activate/minimize under KWin (options in [ADR-0005](../decisions/0005-quickshell-shell.md)). Result written into ADR-0005 |
| T0.6 | Codex | T0.1 | CI on GitHub Actions in an Arch container: `scripts/check_repo.py`, CMake build, `qmllint` on the shell, `cargo test` for `services/policy` |
| T0.7 | Claude | T0.1 | Design tokens: `desktop/design-system/tokens.json` (colors light/dark, radii, spacing, typography, motion) plus a generated `Tokens.qml` singleton. Font chosen (Inter or another open font) |
| T0.8 | Claude | T0.4, T0.7 | `desktop/shell/` skeleton: Quickshell entry point with an empty top bar and bottom bar using the tokens, visible via `dev-nested.sh` |

**Exit gate**
- [ ] Foundation merged into `main`; license chosen
- [ ] `tools/dev-nested.sh` shows the skeleton shell in the dev VM; screenshot attached to the PR
- [ ] CI runs on every PR and is green
- [ ] ADR-0005 updated with the T0.5 spike result (window list approach decided)

---

## Phase 1: Desktop core

**Goal:** a login session called "JadroOS" that already looks like JadroOS.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T1.1 | Claude | T0.8 | Wallpaper layer with an original placeholder wallpaper; light/dark switch |
| T1.2 | Claude | T0.5, T0.8 | Menu bar v1: name of the focused app, fallback app menu (About, Hide, Quit), clock, placeholder status icons |
| T1.3 | Claude | T0.5, T0.8 | Dock v1: pinned apps (from config), running-app dots, click to launch or focus, separator, magnification on hover |
| T1.4 | Claude | T0.7 | KWin window decoration plugin (C++, KDecoration): traffic-light buttons on the left, centered title, light/dark, matching radii and shadows |
| T1.5 | Codex | T0.4 | `platform/session/`: `jadro-session` startup script and `jadroos.desktop` Wayland session (KWin + Quickshell + portals + polkit agent + needed KDE services, without plasmashell). Documented manual install in the dev VM |
| T1.6 | Codex | T1.5 | Default config for the session (`kwinrc`, shortcuts, decoration selected), installed per user, no sudo |

**Exit gate**
- [ ] In the dev VM, choosing "JadroOS" at the login screen starts the session; the Plasma session still works
- [ ] Wallpaper, menu bar with focused app name and clock, Dock that launches and switches apps, traffic-light windows: screenshots in PRs
- [ ] Keyboard focus visible on menu bar and Dock items; reduced motion setting respected

---

## Phase 2: Window management

**Goal:** windows behave the way the requirements say (macOS look, Windows power features).

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T2.1 | Claude | T1.4 | Alt+Tab: KWin task switcher QML theme. Individual windows, most-recent order, previews, Shift reverses, Esc cancels (UX-02) |
| T2.2 | Codex | T1.6 | KWin script: snap layouts, keyboard snapping shortcuts, window group restore |
| T2.3 | Claude | T1.4, T2.2 | Snap layout popup when hovering the green button; edge-drag snapping visuals |
| T2.4 | Claude | T1.3 | Minimize-to-Dock animation (KWin effect) |
| T2.5 | Claude | T1.6 | Mission Control: KWin QML overview effect with a workspace strip, drag windows between workspaces, touchpad gesture |
| T2.6 | Codex | T2.2 | Workspace and shortcut defaults; regression checklist for window behavior |

**Exit gate**
- [ ] UX-02 acceptance passes (Alt+Tab MRU, reverse, Escape, no key grab leaks)
- [ ] Snapping works with mouse and keyboard; Mission Control works with keyboard and touchpad
- [ ] No regressions in Phase 1 features

---

## Phase 3: Spotlight + AI handoff

**Goal:** Spotlight is useful every day: local search, plus AI tabs that hand your question to the official apps. No API calls, no sign-in, no billing yet.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T3.1 | Codex (Claude cross-reviews) | T0.6 | Search contract `org.jadroos.Search1`: D-Bus interface XML, fixtures, errors ([integration-contracts.md](../engineering/integration-contracts.md)) |
| T3.2 | Codex | T3.1 | Search service in Rust (first Rust service): app index from `.desktop` files, settings entries, opt-in file index with SQLite FTS5, systemd user unit |
| T3.3 | Claude | T3.1, T1.2 | Spotlight UI in the shell: overlay with Alt+Space, General tab default, keyboard navigation, mock adapter first |
| T3.4 | Codex | T3.1 | Provider handoff adapter: open official ChatGPT/Claude app or website, "copy prompt" fallback, launch Codex / Claude Code CLI in a terminal in a chosen folder |
| T3.5 | Claude | T3.3, T3.4 | AI tabs UI: ChatGPT (Instant default), Claude, Codex, Claude Code, Local (shows "coming in Phase 7"); clear states for unavailable/copy fallback |
| T3.6 | Claude | T3.3 | General extras: calculator and unit conversion |
| T3.7 | Codex | T3.2, T3.5 | Integration: switch from mock to real service, end-to-end tests |

**Exit gate**
- [ ] UX-03, AI-01 and AI-02 acceptance pass ([requirements.md](../product/requirements.md))
- [ ] Spotlight opens on General every time; typing never makes a network call
- [ ] Warm first paint and result timing measured and recorded (targets in requirements)

---

## Phase 4: System surfaces and polish (first real hardware)

**Goal:** a desktop you could use every day on your own PC.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T4.1 | Claude | T1.2 | Control Center: volume, output device, brightness, Wi-Fi, Bluetooth, Do Not Disturb, power profile |
| T4.2 | Claude | T1.2 | Notifications (Quickshell notification server) and Notification Center, grouped by app |
| T4.3 | Claude (Codex cross-reviews) | T1.5 | Lock screen: themed upstream locker first; custom lock only after security review |
| T4.4 | Codex | T1.5 | Global menu helper: reads the active window's exported menu (KWin app-menu protocol, `appmenu-gtk-module`, Qt KDE integration) and exposes it to the shell |
| T4.5 | Claude | T4.4 | Global menu in the menu bar, with the Phase 1 fallback menu for apps that export nothing |
| T4.6 | Claude | T1.4 | Glass material: KWin blur/tint effect for shell surfaces, solid fallback, frame-time measurements |
| T4.7 | Codex | T0.7 | Theme generator: tokens to Kvantum (Qt apps), GTK3 theme and GTK4 color CSS |
| T4.8 | Claude | T4.7 | Theme content: Qt/GTK styling, original icon set (start with core apps), cursor, sounds |
| T4.9 | Owner | T1.5 | Install Arch on the separate SSD and run the JadroOS session on the RTX 3070 / high-refresh display |

**Exit gate**
- [ ] Session used for a full day on real hardware without falling back to Plasma; issues filed
- [ ] Frame-time measurements for glass at 60 Hz and high refresh on the real GPU, with fallback working
- [ ] Global menu works for Qt, GTK3 and LibreOffice; fallback menu for the rest
- [ ] A11Y-01 checks for all Phase 1–4 surfaces

---

## Phase 5: Distro platform

**Goal:** JadroOS becomes an installable set of packages and a live ISO.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T5.1 | Codex | Phase 4 gate | PKGBUILDs: `jadro-shell`, `jadro-kwin-decoration`, `jadro-kwin-effects`, `jadro-session`, `jadro-themes`, `jadro-search`, meta package `jadro-desktop` |
| T5.2 | Codex | T5.1 | `tools/build-packages.sh`: clean-chroot builds in a container, signed local `[jadro]` repo |
| T5.3 | Codex | T5.2 | `platform/archiso/`: releng-based profile, live ISO that boots into the JadroOS session (live only, no installer) |
| T5.4 | Codex | T5.3 | `tools/run-vm.sh` and `tools/vm-screenshot.sh`: QEMU/OVMF boot with QMP, automated "boots to desktop" check |
| T5.5 | Claude | T0.7 | SDDM login theme and Plymouth boot splash |
| T5.6 | Codex | T5.4 | CI builds packages and ISO on a trusted runner |

**Exit gate**
- [ ] OS-01: ISO boots in a UEFI QEMU VM to the JadroOS desktop, verified by automated screenshot
- [ ] Every package builds in a clean chroot; repo is signed

---

## Phase 6: System apps

**Goal:** the core apps that make it feel like a complete OS.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T6.1 | Claude | T0.7 | Shared QML app component library (sidebar, toolbar, lists, forms, sheets) |
| T6.2 | Claude + Codex | T6.1 | Settings app: UI (Claude), backends for network/display/audio/users/updates (Codex) |
| T6.3 | Claude + Codex | T6.1 | Files app: Finder-style views, Quick Look, tags (Claude); safe transfer queue (Codex) (FS-01) |
| T6.4 | Claude | T6.1 | Welcome / first-run app |
| T6.5 | Claude + Codex | T6.1 | Activity (task manager) and Devices apps (SYS-01) |
| T6.6 | Claude + Codex | T6.1 | App Center basics: pacman, Flatpak, AppImage (APP-01 part 1) |

**Exit gate**
- [ ] FS-01 and SYS-01 core acceptance pass
- [ ] All apps keyboard-accessible, light/dark, included in the ISO

---

## Phase 7: Native AI and agents

**Goal:** AI that works *inside* JadroOS, safely.

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T7.1 | Owner | Phase 3 gate | Verify provider terms and eligibility (OpenAI sign-in, Anthropic, Codex) per the spike criteria in [ai.md](../architecture/ai.md) |
| T7.2 | Codex | T7.1 | Agent service in Rust: provider adapters (Codex app-server, eligible OpenAI sign-in, opt-in API keys, local), streaming, cancel, budgets |
| T7.3 | Claude | T7.2 | Answers inside Spotlight: streaming output, mode selector, usage warnings; Intelligence app (providers, history, consent) |
| T7.4 | Codex (Claude cross-reviews) | T7.2 | Tool broker and system broker (polkit), built on `services/policy`; audit log (SEC-01) |
| T7.5 | Codex + Claude | T7.4 | Screen capture and input through portals with visible consent and a stop button (AI-04) |
| T7.6 | Codex | T7.2 | Local AI (llama.cpp/Ollama) and bounded scheduled jobs (AI-05) |

**Exit gate**
- [ ] AI-03, AI-04, SEC-01 acceptance; adversarial tests (prompt injection, replay, privilege denial, kill switch)
- [ ] No silent paid fallback in any failure test

---

## Phase 8: Windows and gaming

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T8.1 | Codex | T7.4 | Linux "restart into Windows" helper (one-shot UEFI BootNext with readback), tested in a two-disk OVMF VM first |
| T8.2 | Codex | T8.1 | Windows-side "restart into JadroOS" helper |
| T8.3 | Claude | T8.1, T4.1 | "Restart into Windows" in Control Center and Gaming Center, with confirmation |
| T8.4 | Claude + Codex | T6.1 | Gaming Center: Steam/Proton library, compatibility info, per-game profiles |
| T8.5 | Codex | T6.6 | Wine/Bottles per-app prefixes and compatibility tiers in App Center (APP-01 part 2) |
| T8.6 | Owner | T8.1, T8.4 | Physical tests: boot switch on real firmware, NVIDIA games (CS2 native, Proton titles) |

**Exit gate**
- [ ] OS-02 and GAME-01 acceptance; unselected disk untouched in VM tests

---

## Phase 9: Installer, updates and release

| Ticket | Owner | Depends on | Deliverable |
|---|---|---|---|
| T9.1 | Codex + Claude | T5.3 | Calamares installer with JadroOS branding and safe disk selection; two-disk VM tests |
| T9.2 | Codex | T9.1 | Btrfs + snapper: snapshot before each update, rollback from the boot menu (OS-03) |
| T9.3 | Codex | T9.2 | Signed update channel, recovery USB, Secure Boot plan |
| T9.4 | Claude + Codex | T9.2 | Backup app and update UI |
| T9.5 | Claude + Codex | T6.3 | Phone sharing (KDE Connect / LocalSend protocols) (CON-01); Waydroid optional |
| T9.6 | Owner | all | Legal/trademark review, SBOM, third-party notices; public beta, then stable |

**Exit gate (= release)**
- [ ] OS-03, REL-01, CON-01; all release blockers in [testing.md](../engineering/testing.md) clear

---

## "When does feature X happen?"

| Feature | Phase |
|---|---|
| Menu bar, Dock, wallpaper, traffic-light windows | 1 |
| Global app menu in the menu bar | 4 |
| Alt+Tab, snapping, Mission Control, workspaces | 2 |
| Spotlight local search | 3 |
| Spotlight AI tabs (open the official ChatGPT/Claude/Codex/Claude Code) | 3 |
| AI answers directly inside Spotlight, AI agents doing tasks | 7 |
| Local AI models | 7 |
| Control Center, notifications, lock screen | 4 |
| Glass / blur | 4 |
| Qt and GTK apps themed to match | 4 |
| Running on your real PC | 4 |
| Login screen and boot splash | 5 |
| Live ISO | 5 |
| Settings, Files, Activity, Devices, App Center, Welcome | 6 |
| Restart into Windows and back | 8 |
| Gaming Center, Steam/Proton, Wine apps | 8 |
| Installer | 9 |
| Update rollback, backups | 9 |
| Phone sharing | 9 |
| Public release | 9 |

Detailed tickets, statuses and current assignments are on the [task board](task-board.md).
