# Quality gates and merge policy
Status: required engineering policy; automated enforcement not yet configured. Initial CI is split between foundation tickets J-002A/J-002B.

| Gate | Required when | Evidence |
|---|---|---|
| structural/config/schema | every PR | offline repo check and schema validation |
| compile | affected code | clean build and exit status |
| format/lint | affected language | rustfmt, Clippy, clang-format/tidy, QML lint as configured |
| unit + negative cases | behavior changes | actual test logs, new regression for bug |
| integration | interface/service changes | real or explicitly labeled mock tests, contract compatibility |
| security | permissions, external data, dependencies | threat review, denial cases, secret/dependency scan |
| visual/accessibility | UI changes | screenshots from actual build, keyboard and scaling checks |
| performance | hot path/compositor/index changes | baseline and comparable environment measurements |
| VM | platform/boot/installer changes | isolated UEFI VM logs, disk safety proof |
| physical | GPU, gaming, firmware or hardware claims | hardware test record, no mock equivalence |
| cross-review | all substantive changes | review of exact SHA by other agent, resolved findings |
| owner approval | all merges, critical changes | owner explicitly approves |

A check may be not applicable only with written rationale; blocked required check prevents completion. For a docs-only PR, compile may be N/A if code unchanged. Never weaken checks to pass a PR. No known critical/high defects accepted for merge; medium defects require explicit owner disposition and tracking, low defects tracked. Security-critical logic targets thorough branch/negative coverage; proposed backend line coverage goal >=80% once tooling exists, not a substitute for meaningful cases.

## Planned GitHub Actions
Backend job: stable Rust, locked dependencies once lockfile checked in, cargo fmt --check, cargo clippy --all-targets -- -D warnings, cargo test; Python structural checks. Desktop job: supported Qt >=6.5 environment (Arch clean container or pinned Qt setup), CMake/Ninja build, QtTest/QML tests, lint, headless smoke with bounded timeout. Use read-only permissions, no signing or provider secrets, no untrusted PR code on persistent privileged self-hosted runners. Pin actions by reviewed full commit SHA; dependency review/scanning where supported. Separate VM test runner only after isolation and resource constraints designed.

Owner configures GitHub ruleset/branch protection for main after workflows actually run; select real stable required status names, prohibit force pushes/deletions, require PR and up-to-date branch where practical. If agents use same GitHub identity, GitHub approvals may not be technically independent; record cross-review evidence in PR and owner verifies. Do not mark ruleset enabled before checking repository settings.
