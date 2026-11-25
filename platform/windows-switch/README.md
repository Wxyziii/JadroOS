# Windows switch — planned

**When:** Phase 8 (T8.1, T8.2), two-disk VM first. Owner: Codex; UI in Control Center (T8.3) is Claude.

Linux and Windows helpers discover actual firmware entries, validate target, obtain local confirmation, set one-shot BootNext and read back before reboot. Test OVMF then physical UEFI. No hardcoded entry IDs, force reboot, BitLocker bypass or disk modifications. No executable helper provided yet.
