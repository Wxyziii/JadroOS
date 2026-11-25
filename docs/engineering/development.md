# Development guide
## Initial environment
Use a disposable Arch Linux VM, UEFI, 4+ vCPU, 8+ GiB RAM and enough disk for build; use snapshots. On Arch install development dependencies using a full system upgrade, not partial upgrades:
```sh
sudo pacman -Syu --needed base-devel cmake ninja qt6-base qt6-declarative rust python
```
Use distro Qt 6 and stable Rust. These instructions are for a development VM, not the user's main PC. No actual image build or service installation is included.

```sh
cmake -S . -B build -G Ninja
cmake --build build
./build/desktop/spotlight/jadro-spotlight
cargo test --manifest-path services/policy/Cargo.toml
python3 scripts/check_repo.py
```
The Qt app is a normal window, not a registered global shortcut or compositor overlay. It is safe/offline and only displays proposed route text. The Rust policy library contains pure rules and tests, not production authorization. No credentials required.

## Code conventions
C++20 RAII, QML components with explicit accessibility and focus; Rust stable with no unsafe code unless reviewed; typed errors; no shell concatenation; no embedded credentials. Keep UI, providers, policy, system adapters and packaging separate. Use versioned contracts, stable IDs, tests for failures, structured logging. New dependencies require license/security rationale.

## CI design
Run structural checks and Rust tests on ordinary runners; Qt build/smoke in Linux environment with offscreen tests; Arch package build in clean chroot; ISO in isolated VM-capable trusted runner; hardware tests manual. PR jobs have read-only token and no signing secrets. Signing only trusted release branch, environment approval, isolated keys. CI workflow not yet enabled; run checks manually until owner approves runner configuration.

## Packaging/release
Build packages in clean chroot from reviewed source; pin source revision and checksums; sign package DB and artifacts; manifest includes toolchain, versions, hashes, licenses/SBOM and build date. Separate dev/alpha/stable channels. Release checklist: tests, recovery, known limitations, legal notices, signed hashes, install docs, rollback. No distribution license selected yet.
