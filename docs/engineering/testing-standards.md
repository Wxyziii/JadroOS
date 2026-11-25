# Testing standards
Use existing docs/engineering/testing.md for system-wide matrix. Tests must be deterministic, isolated, repeatable and runnable without paid provider requests. Never run destructive tests on main PC.

## Backend
Unit tests for pure policy, parsing, ranking, limits and errors; integration tests for IPC, cancellation, invalid caller and provider mocks. Use temporary directories and fake clocks where appropriate. Rust fmt, Clippy and cargo test; add fuzzing/property tests for parsers and security invariants as project grows. Lock dependencies for reproducible CI. No network or secrets by default.

## Desktop
QtTest/QML Quick Test for component states, keyboard/focus, model change, failed/empty results, stale responses and scale. Compile with supported Qt; headless smoke tests with bounded timeout, real Wayland session manual tests for compositor behavior. Screenshots from actual build with version, resolution, theme, DPI; visual regression threshold reviewed, never replace baseline to hide a bug. Reduced transparency/motion, high contrast and keyboard-only are required for new surfaces. ASan/UBSan for C++ as applicable; TSan separate when concurrency exists.

## Integration
Contract fixtures common to both sides; mocks test UI/backend independently; real service test exercises end-to-end search and launch using safe fixture app, request cancellation and unavailable AI provider. Use separate DBus session and test data; do not modify actual desktop defaults. Security cases: scope denial, spoofed caller, injection, replay, symlink traversal, timeout and kill switch.

## System testing stages
Development VM for builds and nonprivileged desktop testing; separate disposable UEFI VM for boot/install/update tests; two virtual disks for dual boot; physical test PC/separate SSD only after VM gates. VMware GPU does not validate RTX performance, VRR, HDR, anti-cheat or 400 Hz. QEMU/OVMF is target for automated boot testing; nested virtualization is optional and may not work in VMware. Use host-managed VM or isolated runner if needed.

## Result record
Commit SHA, date, OS/kernel, Qt/Rust versions, command, exit code, passed/failed/skipped counts, artifact path or log summary, limitations and follow-up issue. Distinguish not run, skipped, failed and passed. Do not fabricate tests or claim zero bugs.
