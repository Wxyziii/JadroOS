# System broker — planned

**When:** Phase 7 (T7.4), cross-review required. Owner: Codex.

System D-Bus typed operations with caller UID from bus credentials and polkit. No arbitrary root shell. Initial read-only system info; subsequent package/boot operations require human approval and target validation. No systemd unit shipped until implementation and hardening tests.
