# Arch platform, installer, updates and dual boot
## Baseline
Vanilla Arch x86-64, UEFI/GPT; linux + linux-lts fallback, mkinitcpio, systemd, KWin/Wayland, NetworkManager, PipeWire. Start with upstream Arch VM and Plasma fallback. Build a custom Archiso releng-derived profile only after desktop package tests. Use Arch clean chroots and signed pacman packages; publish source and license notices as required. Do not use partial upgrades.

## Disk layout (proposed)
Dedicated Linux SSD preferred. EFI system partition (FAT32, size determined in installer design), encrypted LUKS2 root with Btrfs subvolumes @, @home, @snapshots and selected mutable state; separate recovery backup; swap/zram policy after tests. Choose bootloader after spike (systemd-boot first for UEFI; GRUB if rollback needs warrant). Snapshots exclude mounted Windows volumes and cannot restore firmware or separate ESP automatically. Backup important data and BitLocker recovery key before changes; no automatic resizing or formatting an existing Windows disk.

## Dual boot workflow
1. Read-only discover EFI entries and OS partitions; match Windows Boot Manager by verified EFI path and partition identifier, not label alone. If ambiguous, show choices and stop.
2. Check UEFI support, boot entry existence, current boot state, pending reboot, unsaved-work warning and recovery prerequisites.
3. User selects Windows and explicitly confirms. Privileged helper validates caller, re-enumerates target, writes UEFI BootNext (efibootmgr or equivalent supported API), reads back, then requests logind reboot. If write/readback fails, no reboot; offer cancel and manual instructions.
4. Windows helper uses supported elevated firmware/Bcdedit mechanism after validating actual Linux entry. Test on Windows VM and physical firmware; not all firmware supports one-shot selection reliably.
5. Default boot order unchanged; one-shot target consumed by firmware. If OS fails, use firmware menu or recovery USB.

No literal hardcoded boot entry or automated partition command. Do not turn off Secure Boot/BitLocker as a convenience. Windows fast startup/hibernation may prevent safe NTFS writes; mount read-only if unsafe. No shared Wine/Steam library across OS by default. OS switching is reboot, not live handoff.

## Updates and recovery
Stage and verify packages; preflight disk space, keyring and kernel/driver compatibility; create Btrfs snapshot plus separate ESP/kernel recovery plan; apply full transaction; record package manifest; boot health check. Expose rollback only after tested bootloader/initramfs compatibility; keep last known good kernel and offline USB. User data backup via restic/borg or equivalent separate disk/network, restore test regularly. Snapshots alone are not backups.

## Installation safety
First ISO live-only, no unattended disk writes. Later installer presents exact disks by serial/size, proposed changes and confirmation; explicit opt-in wipe of selected disk. Test with two virtual disks and verify Windows disk hash unchanged. Use OVMF UEFI QEMU, snapshots, no host block device passthrough. Secure Boot signing, enrollment and recovery documented before beta.
