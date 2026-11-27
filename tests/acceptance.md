# Acceptance checklists by phase (none executed yet)

Manual checks that back each phase's exit gate in the [roadmap](../docs/planning/roadmap.md). Tick an item only with recorded evidence (commit, date, environment, screenshot or log).

## Phase 0: Foundation
- [ ] Repo check, CMake build and `cargo test` pass in CI
- [ ] `tools/dev-nested.sh` starts KWin in a window with the shell skeleton; screenshot taken with `tools/screenshot-nested.sh`
- [ ] T0.5 spike result recorded in ADR-0005

## Phase 1: Desktop core
- [ ] "JadroOS" session starts from the login screen in the dev VM; Plasma session still works
- [ ] Menu bar shows the focused app's name and the clock; fallback menu (About, Hide, Quit) works
- [ ] Dock launches pinned apps, shows running dots, focuses running apps
- [ ] Every window has traffic-light buttons; close/minimize/maximize work; light and dark
- [ ] Keyboard focus visible in menu bar and Dock; reduced motion respected

## Phase 2: Window management
- [ ] Alt+Tab: individual windows in MRU order, Shift reverses, Escape cancels and restores focus, no key-grab leaks (UX-02)
- [ ] Snapping by edge drag, keyboard, and the green-button layout popup
- [ ] Minimize-to-Dock animation; Mission Control by keyboard and touchpad; windows dragged between workspaces

## Phase 3: Spotlight + AI handoff
- [ ] Alt+Space opens Spotlight on General every time; keyboard-only operation; screen-reader labels
- [ ] Exact app names rank first; stale queries cancelled; typing makes no network request
- [ ] ChatGPT tab defaults to Instant and opens only the official app/site; never a quota-consuming route
- [ ] Claude, Codex and Claude Code tabs are independent; prompt never placed in URLs, process arguments or logs; failed handoff keeps the prompt
- [ ] Warm first-paint and result timings recorded

## Phase 4: Surfaces and polish
- [ ] Control Center controls volume, output, brightness, Wi-Fi, Bluetooth, Do Not Disturb, power profile
- [ ] Notifications grouped by app; hidden on lock screen; Do Not Disturb works
- [ ] Lock screen passes security review
- [ ] Global menu for Qt, GTK3 and LibreOffice apps; fallback menu for others
- [ ] Glass frame times measured on the real GPU at 60 Hz and high refresh; solid fallback works; reduced transparency works
- [ ] A full day of use on the physical SSD install; mixed-monitor scaling and refresh rates

## Phase 5: Distro platform
- [ ] All packages build in clean chroots; repo signed
- [ ] Live ISO boots in UEFI QEMU to the JadroOS desktop (automated screenshot check)
- [ ] SDDM theme and Plymouth splash shown

## Phase 6: System apps
- [ ] Files: views, Quick Look, tags, transfer queue with pause/retry/conflict prompt, no silent overwrite (FS-01)
- [ ] Settings, Activity, Devices core functions (SYS-01); critical-process termination asks for confirmation
- [ ] App Center installs pacman, Flatpak and AppImage apps; unsupported formats explained
- [ ] Remote access off after install; when on, reachable over Tailscale and closed on every other interface (port scan from LAN)
- [ ] Remote connection needs the remote password; attended mode asks the person at the PC; indicator and Disconnect work; start/end notifications and log entry
- [ ] Remote user reaching the lock screen must enter the password; agents cannot enable remote access
- [ ] Remote app lists tailnet devices and connects to another PC

## Phase 7: Native AI and agents
- [ ] Provider terms and eligibility verified with a test account; no secrets in logs; logout/revoke works
- [ ] Rate-limit and quota-exhaustion paths never fall back to a paid route silently
- [ ] Unknown or out-of-scope tool calls denied by the broker; approvals required for writes and admin actions
- [ ] Prompt injection, symlink escape, replay, privilege denial and kill-switch tests pass
- [ ] Screen capture and input only with visible consent and a stop button

## Phase 8: Windows and gaming
- [ ] Two-disk VM: restart into Windows sets BootNext, reads it back, leaves the unselected disk unchanged
- [ ] Physical dual boot: switch both ways on real firmware
- [ ] Native CS2 and representative Proton titles on NVIDIA; incompatible title offers the Windows restart
- [ ] Game Streaming via Moonlight over Tailscale with NVENC; pairing required; latency and FPS recorded

## Phase 9: Installer, updates, release
- [ ] Installer never touches unselected disks (two-disk VM, disk hash unchanged)
- [ ] Failed update recovers by snapshot rollback; independent backup restore tested
- [ ] Signed packages and ISO, SBOM, legal review, reproducible build inputs
