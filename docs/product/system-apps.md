# System applications (planned)
All applications share original macOS-inspired QML components, keyboard accessibility and service contracts. Reuse proven backends; do not rewrite PDF/media engines or package tools.

| App | Core scope | Backend |
|---|---|---|
| Files | tabs, columns/list/grid, Quick Look, tags, search, editable location, safe transfer queue, undo where possible | KIO or audited filesystem service, SQLite metadata |
| Settings | sidebar, standard/advanced, network, display, audio, privacy, users, updates | NetworkManager, BlueZ, PipeWire, KWin, polkit |
| Activity | process tree, CPU/RAM/GPU, startup, services, logs | procfs, systemd, journal, GPU APIs |
| Devices | hardware, drivers, USB, printers, monitors | udev, sysfs, CUPS, DDC/CI where supported |
| App Center | native/Flatpak/Wine routes, update/removal, permissions | pacman/Flatpak/Bottles adapters |
| Gaming | library, compatibility, overlay, profiles, Windows switch | Steam/Proton/upstream tools |
| Developer | terminals, SSH profiles, containers, VMs, Git, SDKs | OpenSSH, Podman/Docker, libvirt |
| Intelligence | provider settings, task list, history, consent, usage | agent orchestrator and keyring |
| Backup | snapshots, external backup, recovery browser | Btrfs, restic/borg |
| Preview | PDF/image/audio previews, annotations later | existing licensed engines |
| Continuity | paired transfer, clipboard, optional device input | LocalSend/KDE Connect/open protocols |

Files transfer acceptance: pause where feasible, retry, speed/ETA, conflict prompt, no silent overwrite, safe symlink/permissions handling, removable-media failure. Activity process termination requires confirmation for critical/system processes. Device controls show unsupported hardware instead of guessing. No system daemon exposed directly to provider. Notes/calendar/mail/office can be existing applications first; custom replacements later only when justified.
