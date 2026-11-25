# Feature selection matrix
Functional comparison, not a benchmark or exhaustive API inventory. Visual reference is macOS only. Confirmed choices are marked C; other choices proposed P. M=macOS inspiration, W=Windows behavior, L=Linux/native, H=hybrid, X=deferred/limited.

| Area | macOS strength | Windows strength | JadroOS choice | Phase |
|---|---|---|---|---|
| Desktop | Glass, menu bar, Dock | taskbar grouping | M, original branding C | 1 |
| Window switching | app switcher | individual window Alt+Tab | W C | 1 |
| Window layouts | Mission Control/Spaces | Snap Assist, layouts, groups | H; green control hover snap | 2 |
| Workspaces | per-display Spaces | desktop task view | H; per-output experiment | 2 |
| Windows | traffic lights, fullscreen | dedicated maximize semantics | M visuals/W optional behavior | 1 |
| Focus | hot corners, focus modes | notifications, shortcuts | H | 2 |
| File browser | columns, tags, Quick Look | editable location, transfer details | H | 2 |
| Search | Spotlight | indexing, Run | M + local actions C | 1 |
| Clipboard | history/search | pinned history | H, opt-in sensitive filtering | 2 |
| Settings | unified sidebar | advanced admin | H | 2 |
| Task manager | Activity Monitor | process/startup/resource controls | W behavior | 2 |
| Devices/services | System Information | Device Manager, Services | W behavior | 2 |
| Network | sharing and diagnostics | management consoles | H with Linux backends | 2 |
| Audio | simple output controls | per-app volume/routing | W behavior, PipeWire | 2 |
| Displays | gestures, scaling | custom PC/HDR/VRR | H | 2–3 |
| App installs | app bundles/store | EXE/MSI/MSIX, WinGet | L package routing | 3 |
| Gaming | Games, Metal | DirectX, broad catalog | L Steam/Proton + Windows fallback C | 3 |
| Legacy apps | restricted | legacy Windows software | Wine/VM; test each app | 3 |
| Android | no general APK runtime | no universal native runtime | Waydroid optional X | 5 |
| macOS apps | native | not native | X, no general runtime | later |
| Phone sharing | Continuity/AirDrop | Phone Link/Nearby Sharing | H with open protocols | 5 |
| iPhone mirroring | Apple ecosystem | limited iOS support | X, proprietary restriction | later |
| Tablet display | Sidecar | device-dependent | H, experimental | 5 |
| Security | Gatekeeper/privacy | Defender/App Control | L signed packages, sandbox, policy | 1–4 |
| Backups | Time Machine | File History/System Restore | H Btrfs + external backup | 3 |
| Updates | coordinated OS | active hours/rollback | L full Arch upgrades + snapshots | 3 |
| Developer | Unix tooling | PowerShell, RSAT, WSL | L native containers, SSH, pwsh optional | 2 |
| Virtualization | framework | Hyper-V | L KVM/QEMU | 3 |
| PDF/media | Preview, QuickTime | Snipping Tool, Photos | H, reuse engines | 4 |
| Automation | Shortcuts | PowerShell, Power Automate | H agents + systemd timers | 4 |
| AI | Siri/Spotlight | Copilot | C provider tabs and independent agents | 1–4 |
| Accessibility | VoiceOver, reduced motion | Narrator, voice access | H AT-SPI and keyboard | 1–5 |

## Selection rationale
Choose macOS layout, controls, motion, hierarchy, materials and spacing; choose Windows semantics when they improve power-user workflows; choose upstream Linux mechanisms instead of imitating proprietary APIs. Keep compatibility claims scoped to tested versions. Windows remains the fallback for applications with unsupported anti-cheat, DRM, GPU APIs or drivers. Feature priorities are not promises of completion.
