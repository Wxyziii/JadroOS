# Remote access service (planned)

**When:** Phase 6 (T6.7), game streaming in Phase 8 (T8.7). Cross-review required. Owner: Codex.

`jadro-remote` user service (Rust): reads Tailscale status through its local API, starts and stops KRdp (and later Sunshine), tracks active sessions, writes the connection log and exposes `org.jadroos.Remote1` on the session D-Bus for Settings, the menu bar indicator and the Remote app. It never opens ports outside `tailscale0`, never stores the remote password outside the keyring, and refuses requests from the agent service. Design and security rules: [remote-access.md](../../docs/architecture/remote-access.md).
