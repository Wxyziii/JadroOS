# Tools (planned)

Developer and VM scripts. Owner: Codex. Agents use these to see and check their own work.

| Script | What it does | Ticket | Phase |
|---|---|---|---|
| `dev-nested.sh` | Start `kwin_wayland --windowed` with `desktop/shell` loaded in Quickshell | T0.4 | 0 |
| `screenshot-nested.sh` | Save a PNG of the nested session | T0.4 | 0 |
| `build-packages.sh` | Clean-chroot package builds in a container, signed local repo | T5.2 | 5 |
| `build-iso.sh` | Build the live ISO from `platform/archiso` | T5.3 | 5 |
| `run-vm.sh` | Boot the ISO in QEMU/OVMF with a QMP socket | T5.4 | 5 |
| `vm-screenshot.sh` | Screenshot the VM through QMP; "boots to desktop" check | T5.4 | 5 |

Rules: no `sudo` inside scripts agents run; anything needing root is printed as an instruction for the owner. Scripts fail loudly with clear messages.
