# Integration contract workflow
Status: initial design, not a running IPC service. Prefer typed interfaces and versioned fixtures to tightly coupled source changes. Contract PR is merged before dependent backend/UI work; UI can use mock responses while backend is built. Owner designates one contract author; both agents review.

## Spotlight first vertical slice
General search interface proposed org.jadroos.Search1 on session bus. Request: query string, generation unsigned integer, limit bounded 1..50. Response: same generation, ordered records with stable id, kind, title, subtitle, icon identifier and opaque action token. Errors: INVALID_QUERY, NOT_AVAILABLE, CANCELLED, STALE_GENERATION, INTERNAL. No shell string returned or executed. Token must be resolved and validated server-side against current app index. Exact interface signatures, introspection XML and fixtures are J-004A deliverables and require review before integration; this prose is not frozen ABI.

Provider route: see protocols/agent-request.v1.schema.json and docs/design/spotlight.md. General never calls network. ChatGPT Instant official handoff only, no automatic fallback. Unavailable advanced native capability produces explicit unavailable or user-selected official UI path. Agent never trusts UI-supplied authorization fields. Result events include request ID, generation, status, optional text; no sensitive data in status log.

## Shared fixtures and compatibility
Author defines types, validation, error paths, max sizes, ownership, caller identity and cancellation. Both agents agree on example success/error/cancel/stale fixtures and contract tests. UI handles empty and failure states; backend rejects invalid input and old generation. Add fields compatibly; breaking change increments protocol version and migration plan. Mock behavior clearly labeled in UI and tests.

## Integration handoff
Backend PR includes interface spec, runnable test harness, invocation instructions and sample results. UI PR includes mock adapter, component tests and screenshot evidence. Integration ticket switches mock to real adapter in dedicated branch; validate on Arch Wayland session with real and failure cases. Run both suites and trace request IDs across UI/service, no tokens in logs.
