# Testing

What gets tested, where, and from which phase on. A result only counts once it has actually been run and recorded (commit, date, environment, command, result).

## Where each kind of test runs

| Environment | Used for | From phase |
|---|---|---|
| **Nested session** in the dev VM (`tools/dev-nested.sh`) | Shell and KWin work: looks, keyboard, behavior. Agents check their work with `tools/screenshot-nested.sh` | 0 |
| **CI** (GitHub Actions, Arch container) | Repo checks, builds, unit tests, QML lint, headless UI tests | 0 |
| **Dev VM login session** | The full JadroOS session from the login screen; Plasma fallback | 1 |
| **Real PC on the separate SSD** | Glass, animation smoothness, NVIDIA, high refresh rates, multi-monitor, daily use | 4 |
| **Disposable QEMU/OVMF test VM** | ISO boot, automated "boots to desktop" screenshot checks | 5 |
| **Two-disk QEMU VM** | Windows boot switch, installer disk safety, rollback | 8, 9 |
| **Physical dual boot** | Boot switch on real firmware, games on NVIDIA | 8 (after VM gate) |

The VMware dev VM does **not** prove GPU performance, VRR/HDR, anti-cheat or high-refresh behavior. Those claims need the real-hardware tests.

## Kinds of tests

| Level | Examples |
|---|---|
| Static | Formatting, lint (`qmllint`, `clang-format`, `rustfmt`, Clippy), secret scan, `scripts/check_repo.py` (required files, broken links, safe defaults) |
| Unit | Search ranking, `.desktop` parsing, provider routing, policy classification, argument validation, cancellation |
| UI | Qt Quick Test for components: keyboard and focus order, empty/error states, light/dark, scaling, reduced motion/transparency |
| Visual | Screenshots of the actual build with resolution, scale and theme noted; compared side by side with the macOS reference gallery. Baselines are never replaced just to hide a bug |
| Integration | D-Bus contract tests with shared fixtures, mock vs. real service, isolated D-Bus session and test data |
| Security (Phase 7+) | Prompt injection, spoofed caller, replay, symlink traversal, privilege denial, kill switch, quota exhaustion |
| VM / platform (Phase 5+) | UEFI boot, live ISO, update failure, rollback, boot target readback, unselected disk unchanged |
| Physical (Phase 4+) | GPU frame times, high refresh, suspend/resume, USB/audio, games |

## Per-language basics

- **QML / C++:** build with CMake + Ninja, Qt Quick Test for components, headless runs use `QT_QPA_PLATFORM=offscreen` with a time limit. ASan/UBSan for C++ where practical.
- **Rust (Phase 3+):** `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, locked dependencies. Property/fuzz tests for parsers and security rules as they grow.
- **Python/shell scripts:** run in CI; no network or secrets needed.

Tests must be deterministic and runnable without paid AI requests, network access or real credentials.

## CI

Introduced in T0.6, extended as components appear.

| Job | Starts in | Contents |
|---|---|---|
| repo | 0 | `scripts/check_repo.py` |
| desktop | 0 | CMake build, `qmllint` on `desktop/shell`, Qt Quick tests (offscreen) |
| services | 0 | `cargo test` for `services/policy`; later every Rust service |
| packages | 5 | Clean-chroot package builds |
| iso | 5 | archiso build + QEMU boot screenshot (trusted runner only) |

CI rules: read-only token on PRs, no signing or provider secrets, actions pinned to full commit SHAs, untrusted PR code never runs on a persistent privileged runner. The owner turns on branch protection for `main` once the jobs run reliably.

## Performance

Report p50/p95 over 30+ runs, warm and cold separately, with hardware, driver, refresh rate and power profile. Frame budget is ~16.7 ms at 60 Hz, ~6.9 ms at 144 Hz, ~2.5 ms at 400 Hz. Glass quality adapts to the budget instead of promising every effect at every refresh rate. Spotlight targets: warm first paint p95 < 100 ms, local app results p95 < 150 ms.

## Hardware matrix

- QEMU/OVMF with virtio display/disk, two independent disk images (Phase 5+)
- Owner's PC: Intel i9-14900KF, RTX 3070, 32 GB, 1080p high refresh plus mixed monitors (Phase 4+)
- Before public beta: an AMD GPU system, an Intel iGPU laptop, a mixed-DPI laptop + external display

## Release blockers

Any unintended disk write, privilege bypass, leaked credential, silent paid fallback, broken lock screen, missing recovery path or severe accessibility regression. Manual acceptance checks per phase are in [tests/acceptance.md](../../tests/acceptance.md).
