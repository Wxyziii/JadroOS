# Desktop experience and visual system
Only real Apple macOS reference imagery is permitted. Use macOS 27 Golden Gate as current baseline, Tahoe 26 for detailed published imagery. No pearOS, WhiteSur, third-party KDE themes, Windows screenshots or imitation assets as visual references. See [gallery](references.md).

## Design system
Original Jadro iconography and redistributable fonts; no Apple logo, SF fonts, proprietary wallpapers or copied assets. Aim for close hierarchy, proportions, materials and motion, not a promise of pixel-identical branding. Initial token hypotheses: 4/8/12/16/24/32 logical-pixel spacing, 12–24 radii for floating surfaces, 150–250 ms transitions, 44 logical-pixel large targets, visible focus; tune against reference and accessibility tests. Light/dark, clear/tinted material, high contrast, reduced motion/transparency. Use Noto Sans or another reviewed open font; record its license.

## Surface specifications
| Surface | Intended behavior |
|---|---|
| Menu bar | global app menu when app exports supported actions; fallback app name and local window menu; clock/status/control center |
| Dock | pinned/running apps, separators, badges, minimize/restore, magnification, auto-hide, per-output rules |
| Desktop | original wallpapers, widgets opt-in, file icons configurable |
| Window decoration | left traffic-light controls; close/minimize/maximize; green hover exposes snap layouts |
| Snap | drag to edges, keyboard actions, group restoration; macOS-style popup visuals |
| Alt+Tab | MRU individual windows including windows of same app; previews; Shift reverses; Escape restores focus; no minimized-window surprises |
| Mission Control | overview of desktops/windows, drag between workspaces, optional Stage Manager later |
| Control Center | audio, display, network, Bluetooth, focus, power, Windows switch with confirmation |
| Notifications | grouped by app, privacy on lock screen, dismiss/do-not-disturb, accessible actions |
| Lock screen | secure upstream locker first; custom aesthetics only after security review |
| Spotlight | see dedicated spec; no context capture without consent |

## Glass material engineering
Qt Quick renders in-app glass against known content using shader effects; real cross-window backdrop blur/refraction requires compositor-owned sampling. Do not capture other clients from a normal app or fake secure surfaces. Spike compositor effect with clipped blur, downsample, refraction, tint, edge highlights, noise and contrast adaptation; never sample lock/protected content. Cache static backgrounds, damage-based rendering, GPU profiling, HDR linear-space checks, fallback solid tint when unsupported. Test mixed DPI and multiple refresh rates. No claim of exact Apple shader implementation.

## Interaction rules
Logical px not physical px; scaling 100/125/150/200%. Keyboard-first and AT-SPI names; no essential information conveyed by color alone. Reduced transparency changes material not layout. Fullscreen game suppresses overlays unless explicitly opened. Accessibility and performance override fidelity if needed. All shortcuts configurable; avoid collisions with compositor/app shortcuts.

## Acceptance
Reference gallery reviewed side-by-side at equivalent viewport; record original screenshots of Jadro only when actually implemented. Test focus order, high contrast, keyboard-only, screen reader, multi-monitor, touchpad gestures, fullscreen and suspend/resume. Use accessible text contrast targets 4.5:1 normal, 3:1 large; measure on worst-case moving backdrop. No Jadro screenshot is presented as real macOS.
