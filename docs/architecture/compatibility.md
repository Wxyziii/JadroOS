# App compatibility and gaming

**When:** pacman, Flatpak and AppImage in the App Center in Phase 6 (T6.6); Wine/Bottles, Gaming Center and Windows fallback in Phase 8 (T8.4–T8.6). Game streaming to other devices (Sunshine/Moonlight) is part of [remote access](remote-access.md), Phase 8 (T8.7). See the [roadmap](../planning/roadmap.md).
The installer can recognize many formats; it cannot execute every app. Provide per-app compatibility state: native/tested, works with runtime, experimental, unsupported, unknown. Record app version, runtime version, GPU/driver, test date and known limitations.

| Format | Runtime | Rules |
|---|---|---|
| Arch pkg.tar.zst | pacman | signed repository; explicit local package trust review |
| Flatpak / flatpakref | Flatpak | show remote and permissions |
| AppImage | user execution | verify provenance; sandbox where possible |
| deb / rpm | isolated distro container or VM | never install into Arch root automatically |
| exe / msi | Wine/Bottles | separate prefix per app, no guaranteed drivers |
| msix | Windows VM/Windows | conditional, no universal Wine support |
| apk | Waydroid optional | GPU/kernel compatibility spike, not default |
| dmg / app | inspect/extract only when lawful and supported | macOS binaries generally unsupported |
| sh / binaries | explicit run | show source, no auto-execute or root |

## Gaming Center
Integrate installed Steam titles first; Proton version, compatibility metadata, gamepad, audio, shader cache, launch options and per-game profiles. Later add Heroic/Lutris and original macOS-inspired game library. Use MangoHud/Gamescope when compatible; overlays opt-in. FPS/frame time/CPU/GPU/VRAM and recording via upstream tools; instant replay consumes resources. Validate native and Proton titles individually; anti-cheat and DRM can be unsupported, never bypass them. Windows switch available from incompatible title, with reboot confirmation; no claim of automatically launching Windows game until helper tested.

## Hardware focus
Reference physical profile: Intel i9-14900KF, RTX 3070, 32 GB RAM, 1080p high-refresh (up to 400 Hz) plus mixed displays. Test NVIDIA proprietary driver and supported open kernel module variants separately; i9 KF lacks integrated graphics, so passthrough/single-GPU VM is not an MVP solution. CS2 native Linux test target; Valorant Windows fallback, subject to current vendor requirements. No frame-rate guarantee. Include AMD/Intel and laptop reference test coverage before broad release.

## App security
Untrusted installers not executed by default; preview source, signatures and required access. AUR recipes are user-reviewable, not trusted binaries. No silent Windows license acquisition; VM requires user-provided license and resources. No copying proprietary macOS frameworks or Windows files into distribution.
