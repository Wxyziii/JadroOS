# Technology stack

What JadroOS is built with, and from which [roadmap](../planning/roadmap.md) phase each piece is needed. Exact versions get pinned in a release manifest; Arch is rolling, so only full upgrades are supported.

| Layer | Choice | Starts in phase | Why / alternative |
|---|---|---|---|
| OS base | Vanilla Arch, pacman, systemd | 0 | Full control; CachyOS tuning optional later, not a dependency |
| Kernel | Arch `linux`, `linux-lts` fallback | 0 | Upstream maintenance; no custom kernel |
| Window manager / compositor | **KWin (Wayland) + XWayland**, Plasma session as fallback | 0 | Mature floating windows, snapping, effects, NVIDIA, multi-monitor, HiDPI ([ADR-0001](../decisions/0001-foundation.md)) |
| Shell host | **Quickshell** (QML) | 0 | Layer-shell surfaces, notifications, tray, PipeWire, session lock, live reload ([ADR-0005](../decisions/0005-quickshell-shell.md)). Fallback: own Qt app + LayerShellQt |
| Window list in shell | Decided by spike T0.5 (KDE task manager module / protocol / KWin script) | 0 | See ADR-0005 |
| Design system | `tokens.json` → generated `Tokens.qml`, shared QML components | 0 | One source for shell, apps, themes, login screen |
| Window decoration | KDecoration plugin (C++) | 1 | Traffic-light buttons and title bars on every window |
| Session | Own `jadroos.desktop` Wayland session: KWin + Quickshell + portals + polkit agent | 1 | Replaces plasmashell, keeps KDE plumbing |
| KWin effects / task switcher | QML effects and task switcher theme, KWin scripts | 2 | Alt+Tab, Mission Control, minimize-to-Dock, snap layouts |
| Search service | Rust, D-Bus (`org.jadroos.Search1`), SQLite FTS5, XDG desktop entries | 3 | Local-first; first Rust service ([ADR-0007](../decisions/0007-language-policy.md)) |
| AI handoff | Open official ChatGPT/Claude app or site; launch Codex / Claude Code CLI | 3 | No credentials or billing in Phase 3 |
| Global menu | KWin app-menu protocol, `appmenu-gtk-module`, Qt KDE integration, own helper | 4 | Menus from Qt, GTK3, LibreOffice; fallback menu for others |
| Glass | KWin blur/tint effect, solid fallback | 4 | Real cross-window blur needs the compositor |
| App theming | Kvantum (Qt), GTK3 theme + GTK4 color CSS, original icons, open font (Inter or similar) | 4 | Generated from tokens |
| Audio / network / devices | PipeWire/WirePlumber, NetworkManager, BlueZ, UPower, power-profiles-daemon, CUPS | 4 | Standard service APIs (Control Center, later Settings) |
| Packaging | PKGBUILDs, `devtools` clean chroots, signed `[jadro]` repo | 5 | Reproducible, signed |
| Live ISO | archiso (releng-based) | 5 | Standard Arch tooling |
| Login / boot visuals | SDDM QML theme, Plymouth theme | 5 | Same language as the shell |
| VM testing | QEMU/KVM + OVMF, QMP screenshots | 5 | Automated "boots to desktop" checks |
| System apps | Qt 6 / QML apps with C++ or Rust backends, KIO for file operations | 6 | Settings, Files, Activity, Devices, App Center, Welcome |
| Remote access | Tailscale, KRdp (RDP server via portals), RDP client in the Remote app; Sunshine + Moonlight for streaming | 6, 8 | Tailnet-only, standard clients on every platform ([remote-access.md](remote-access.md)) |
| App formats | pacman, Flatpak, AppImage (Phase 6); Wine/Bottles, Distrobox (Phase 8) | 6, 8 | Separate runtimes per format ([compatibility.md](compatibility.md)) |
| Agent service | Rust (zbus, Tokio, serde), provider adapters | 7 | Memory safety for network-facing code |
| Privilege | polkit, systemd, D-Bus typed broker using `services/policy` | 7 | No arbitrary root API ([ADR-0003](../decisions/0003-privilege.md)) |
| Local AI | llama.cpp or Ollama, optional | 7 | No account needed |
| Gaming | Steam/Proton, DXVK/VKD3D, Mesa/NVIDIA, MangoHud, Gamescope | 8 | Upstream compatibility |
| Windows switch | UEFI BootNext helper (Linux, Rust) + Windows helper | 8 | One-shot boot, no boot-order changes |
| Virtualization | KVM/QEMU/libvirt | 8 | Windows VM fallback for some apps |
| Installer | Calamares with JadroOS branding | 9 | Standard for Arch derivatives, QML-themable |
| Snapshots / rollback | Btrfs + snapper, bootloader integration | 9 | Undo bad updates |
| Security | LUKS2, nftables, portals/sandboxing, Secure Boot | 9 | Opt-in secure setup |
| Phone sharing | KDE Connect / LocalSend protocols | 9 | Open protocols; no iOS parity promise |
| Tests | Qt Quick Test, CTest, cargo test, pytest where needed, QEMU | 0+ | See [testing.md](../engineering/testing.md) |
| CI | GitHub Actions in Arch containers; trusted runner for ISO | 0, 5 | No secrets in PR jobs |
| Docs | Markdown, Mermaid | 0 | Versioned with the code |

## Decision gates
Window list under KWin (T0.5) · Quickshell + KWin version pairing (Phase 1) · glass performance on real GPU (T4.6/T4.9) · reproducible archiso boot (T5.3) · unattended remote consent (T6.7) · provider terms and auth (T7.1) · polkit broker review (T7.4) · NVIDIA HDR/VRR (Phase 8) · signed updates and rollback (T9.2–T9.3). Every dependency is license-checked before distribution.
