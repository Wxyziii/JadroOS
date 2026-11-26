# System applications (planned)

Phase numbers refer to the [roadmap](../planning/roadmap.md). All apps share the design-system component library (T6.1).
All applications share original macOS-inspired QML components, keyboard accessibility and service contracts. Reuse proven backends; do not rewrite PDF/media engines or package tools.

| App | Core scope | Backend | Phase |
|---|---|---|---|
| Files | tabs, columns/list/grid, Quick Look, tags, search, editable location, safe transfer queue, undo where possible | KIO or audited filesystem service, SQLite metadata | 6 |
| Settings | sidebar, standard/advanced, network, display, audio, privacy, users, updates | NetworkManager, BlueZ, PipeWire, KWin, polkit | 6 |
| Activity | process tree, CPU/RAM/GPU, startup, services, logs | procfs, systemd, journal, GPU APIs | 6 |
| Devices | hardware, drivers, USB, printers, monitors | udev, sysfs, CUPS, DDC/CI where supported | 6 |
| App Center | native/Flatpak/Wine routes, update/removal, permissions | pacman/Flatpak/Bottles adapters | 6 (native/Flatpak/AppImage), 8 (Wine) |
| Gaming | library, compatibility, overlay, profiles, Windows switch | Steam/Proton/upstream tools | 8 |
| Developer | terminals, SSH profiles, containers, VMs, Git, SDKs | OpenSSH, Podman/Docker, libvirt | after 6 |
| Intelligence | provider settings, task list, history, consent, usage | agent orchestrator and keyring | 7 |
| Backup | snapshots, external backup, recovery browser | Btrfs, restic/borg | 9 |
| Preview | PDF/image/audio previews, annotations later | existing licensed engines | 6 |
| Welcome | first-run setup: account, appearance, privacy, AI providers intro | Settings backends | 6 |
| Remote | connect to other tailnet devices (RDP, Moonlight), favorites; host settings live in Settings → Remote Access | Tailscale, KRdp, RDP client, Moonlight, jadro-remote service | 6 (streaming 8) |
| Continuity | paired transfer, clipboard, optional device input | LocalSend/KDE Connect/open protocols | 9 |

Files transfer acceptance: pause where feasible, retry, speed/ETA, conflict prompt, no silent overwrite, safe symlink/permissions handling, removable-media failure. Activity process termination requires confirmation for critical/system processes. Device controls show unsupported hardware instead of guessing. No system daemon exposed directly to provider. Notes/calendar/mail/office can be existing applications first; custom replacements later only when justified.
