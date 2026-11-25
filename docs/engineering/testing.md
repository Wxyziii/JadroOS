# Verification strategy
No test result is claimed until executed and logged with commit, environment and date.

| Level | Coverage |
|---|---|
| Static | format/lint, dependency/license review, secret scan, schema and link checks |
| Unit | routing, ranking, policy classification, args validation, cancellation, model capabilities |
| Integration | D-Bus caller identity, portal denial, provider mock streams, keyring failure, file conflicts |
| UI | keyboard, focus, screen reader, scale, light/dark, contrast, reduced motion |
| VM | UEFI boot, two disks, live ISO, update failure, rollback, boot target readback |
| Physical | NVIDIA/AMD/Intel GPU, high refresh, HDR/VRR, suspend, USB/audio, gaming |
| Security | injected tool proposals, traversal/symlinks, replay, quota, malicious app, revoke |
| Release | signed artifacts, clean install, upgrade, offline recovery, backup restore |

## Hardware matrix
- QEMU/OVMF baseline, virtio display/disk, two independent disk images.
- Intel i9-14900KF + RTX 3070, 32 GB; 1080p high refresh and mixed monitors. No disk passthrough to initial VM.
- AMD GPU + CPU desktop, Intel iGPU laptop, mixed DPI laptop/external display (additional hardware required).
- Windows VM and physical dual-boot separate SSD for firmware tests after VM pass.

## Performance protocol
Report p50/p95/p99 with 30+ trials, warm/cold separated, hardware/driver/refresh, power profile, background load. Spotlight warm first paint <100 ms and app results <150 ms proposed. Measure frame time, dropped frames, VRAM, CPU, idle memory. At 60/144/400 Hz budget is ~16.7/6.9/2.5 ms respectively; adapt glass quality instead of promising all effects at high refresh.

## Release blockers
Any unintended disk writes, privilege bypass, leaked credential, unsupported billing fallback, broken lock screen, no recovery path, or severe accessibility regression. Manual gates in tests/acceptance.md. Run provider tests with explicit test account and no sensitive data.
