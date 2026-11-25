# Technology stack and selection
Pin exact versions in a release manifest after compatibility spike; Arch rolling release means unsupported partial upgrades must be avoided.

| Layer | Proposed stack | Why / alternative |
|---|---|---|
| OS | vanilla Arch, pacman, systemd | control of distro; CachyOS tuning optional, not a dependency |
| Kernel | Arch linux and linux-lts fallback | upstream maintenance; no custom fork initially |
| Build | archiso releng, makepkg, devtools clean chroots | reproducible inputs and signed repository |
| Session | KWin Wayland + XWayland, Plasma fallback | mature NVIDIA and multi-output stack; no prebuilt visual theme |
| UI | C++20, Qt 6/QML, CMake/Ninja | native Qt render path, accessibility; Rust UI alternative deferred |
| Design | original QML components, QSB shaders, KWin effects | compositor backdrop access only through supported interfaces |
| Services | Rust stable, zbus/Tokio/serde after spike | memory safety, async IPC; no heavy framework requirement |
| Privilege | polkit, systemd, D-Bus | typed authorization; no arbitrary root terminal API |
| Search | SQLite FTS5, XDG desktop entries, incremental indexing | local-first; optional embeddings later |
| Audio | PipeWire/WirePlumber | streams, routing and capture |
| Network | NetworkManager, BlueZ, CUPS | standard service APIs |
| Security | LUKS2, nftables, sandboxing/portals, Secure Boot | opt-in secure setup and explicit trust model |
| Apps | pacman, Flatpak, AppImage, Distrobox optional, Wine | distinct install/runtime environments |
| Gaming | Steam/Proton, DXVK/VKD3D, Mesa/NVIDIA, MangoHud | upstream compatibility and metrics |
| VM | KVM/QEMU/libvirt, virtio | Windows fallback apps, OVMF VM tests |
| Local AI | llama.cpp or Ollama optional | no account requirement; resource constraints |
| AI providers | official web/native handoff, SIWC preview, Codex app-server, Claude Code official workflow | separate authentication and usage contracts |
| Testing | CTest/QtTest, cargo test, pytest where needed, QEMU | reproducible unit/VM/integration tests |
| CI | GitHub Actions (proposed), Arch isolated runners for ISO | no secrets in untrusted PR jobs |
| Docs | Markdown, Mermaid, original SVG diagrams | versioned alongside source |

Decision gates: compositor API/Wayland focus; official prompt handoff; provider auth and model capability discovery; Rust IPC/polkit; archiso reproducible boot; NVIDIA HDR/VRR; signed updates. Record exact package versions and checksums per build. License audit every dependency before distribution.
