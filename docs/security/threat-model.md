# Threat model and security requirements
Trust boundaries: logged-in human, UI/session processes, untrusted providers/model output, untrusted files/webpages, tool broker, privileged OS broker, firmware, upstream package supply chain. Root is not a normal agent role.

| Threat | Mitigation and test |
|---|---|
| Prompt injection in file/web output | mark external text untrusted; typed calls, allowlist, no policy changes from content; adversarial corpus |
| Confused deputy | derive caller UID/session from IPC; validate target and scope; polkit interactive challenge |
| Secret exfiltration | keyring, no raw env dumps, attachment preview, redact logs, network controls |
| Arbitrary command execution | argv not shell, sandbox workspace, no arbitrary root; reject path traversal and symlink escapes |
| Approval replay | operation ID, canonical arguments hash, uid/session, scope, expiry, single use; reject altered target |
| Desktop input abuse | explicit portal consent, visible indicator, kill switch, deny lock/credential surfaces |
| Firmware/disk damage | separate helper, validate EFI GUID/path, confirmation, no automatic partition operations |
| Supply chain | signed packages, pinned release inputs, dependency audit/SBOM, no untrusted AUR automatic installation |
| Privilege escalation | fixed operation set, service hardening, least privilege; no sudo NOPASSWD |
| Sensitive audit logs | metadata only by default, bounded retention, user export/delete; no prompts/tokens/passwords |
| Denial of service | job quotas, resource cgroups, deadline, cancel, backoff |
| Malicious update | verify signature, snapshot/rollback, independent recovery media |

## Permissions
Read scoped non-sensitive data may be session-granted; writes require explicit scope and preview; package installs, network config, reboot, destructive file operations and firmware changes require just-in-time approval. Deny by default on unknown capability or missing consent; deny credential access and security bypass regardless of model request. Approval is issued by trusted local UI, not a provider or caller-provided boolean. The starter crate models a decision only, not a real authorization implementation.

## Platform security
LUKS2 optional at install with verified recovery key; UEFI Secure Boot after tested signing/enrollment path; do not disable Windows security features to make Linux boot. Use portal/Flatpak isolation, systemd service hardening, nftables, patch cadence and minimal services. Recovery boot and key enrollment require manual verification. No security claims until negative tests and independent review.

## Privacy defaults
No telemetry, cloud history, screen capture, clipboard history, file indexing outside selected scope, or background agents by default. Clear data retention controls. Consent may be revoked; running jobs stop and grants invalidated. Incident process: contain, revoke keys, preserve redacted evidence, patch, notify affected users according to applicable obligations.

**When security reviews happen:** lock screen (T4.3), search contract (T3.1), agent/tool/system broker (T7.4), screen capture and input (T7.5), remote access (T6.7, T8.7), boot switch (T8.1), installer and updates (T9.1–T9.3). Each is a cross-reviewed ticket ([workflow.md](../engineering/workflow.md)).

## Remote access (Phase 6, 8)
See [remote-access.md](../architecture/remote-access.md). Threats and mitigations: exposure beyond the tailnet (ports only on `tailscale0`, tested on every other interface; no Funnel or port forwarding); stolen tailnet device (second factor: remote password or paired client; Tailscale ACLs; revoke from Settings); silent watching (portal consent, always-visible indicator, start/end notifications, connection log); lock-screen bypass (remote sessions land on the normal lock screen); agent abuse (the agent service cannot enable remote access). Remote access is a cross-reviewed ticket (T6.7, T8.7).
