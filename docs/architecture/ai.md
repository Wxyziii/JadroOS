# AI platform and provider contracts
Status: planned; routing library is a local reference only.

**When:** Phase 3 builds only the *handoff* routes (open the official ChatGPT/Claude app or site, launch the Codex / Claude Code CLI). Everything else on this page (sign-in, native inference, agents, tool broker, local AI) is Phase 7 and starts with the owner verifying provider terms (T7.1). See the [roadmap](../planning/roadmap.md).
 Revalidate provider terms and capabilities before enabling real requests.

## Provider matrix
| Route | Auth | Usage | Context |
|---|---|---|---|
| Official ChatGPT Instant | user signs into official app/site | official Instant rules | existing chats/memory/projects as supported |
| Official ChatGPT Thinking | official account | plan rules | existing context |
| Native OpenAI SIWC | documented OAuth/PKCE registration | eligible ChatGPT plan allowance; currently Work/Codex | app-owned history, not ChatGPT account history |
| OpenAI API key | user-provided key, explicit opt-in | separate API billing | app-owned |
| Codex app-server/CLI | supported official auth | plan/API depending on mode | local threads/projects and sandbox |
| Claude chat | official app/site | Claude plan rules | official session |
| Claude Code | official CLI login | Claude terms/plan | user-driven CLI project |
| Anthropic SDK | supported API key/cloud auth | separate billing | app-owned |
| Local | no provider account | device resources | local |

As of 2026-11-25 OpenAI announces December shared usage for eligible Plus/Pro/Business Thinking, Work and Codex; Instant chat is excluded. This exemption is not established for SIWC. Do not represent native inference as free Instant. Verify account eligibility and model list dynamically; no hardcoded future quota or model IDs. Anthropic documentation limits subscription OAuth to ordinary native Claude/Claude Code use; third-party SDK integrations need supported API/cloud authentication. Never reuse CLI OAuth tokens in a custom inference client.

## Provider interface
Methods: capabilities(), authenticate(), submit(request), cancel(request_id), logout(). Capability includes supported modes, attachment types, streaming, tools, usage category, availability, last verified date. Events: queued, running, text_delta, tool_proposed, approval_required, tool_result, usage, completed, failed, cancelled. Error codes: NOT_AVAILABLE, AUTH_REQUIRED, NOT_ELIGIBLE, MODE_UNSUPPORTED, RATE_LIMITED, POLICY_DENIED, TIMEOUT, CANCELLED. Treat provider usage as reported/unknown, not guessed.

## SIWC spike
Use current official registration flow, OAuth state/PKCE S256, redirect binding, validate ID token issuer/audience/nonce/expiry and grant; per-install host identifier; access/refresh credentials in keyring; refresh rotation atomic; revoke/logout; dynamic model discovery. Use documented Responses endpoint, streaming parser and provider-specific error handling. No browser cookie extraction, unauthorized endpoint or general API proxy. Preview limitations may include hosted tools; implement only allowed functions. No background access without explicit grant. Do not ship secrets or client credentials in repo.

## Agent execution
Plan -> user scope -> provider call -> tool proposal -> schema validation -> policy -> approval (if required) -> isolated execution -> audit -> result -> budget check -> finish. Bound time, tokens, parallelism, tool calls and cost; cancellation terminates child process group. Read-only first, preview writes, immutable request ID and audit correlation. Agent does not receive credentials. Local shell access only inside explicit user workspace sandbox; direct terminal mode explicit, no permanent root. Agent text cannot change security policies. Multi-agent delegation inherits narrower scopes; no agent can approve another agent's escalation.

## Tool catalog (planned)
system.info/read_logs, files.list/read/write, apps.launch, network.diagnose, terminal.run_workspace, desktop.capture/click/type, packages.install, boot.switch. Each tool defines input/output schema, risk, scope, timeout and rollback semantics. Screen capture via PipeWire/portal; input via RemoteDesktop portal or trusted compositor extension with consent; AT-SPI semantic operations preferred. Do not use unrestricted uinput, root shell or X11 keylogging. No interaction with lock screen, password fields, payment or MFA prompts. Stop button and visible active-session indicator.

## Data handling
Prompt/attachments preview, redact secrets, opt-in file/screen sharing; provider destination shown. Store history locally only if enabled. Never send general search index, clipboard or system logs automatically. Provider network allowlist where feasible; untrusted web content is data, never instructions. Budget warnings before expensive tasks; no automatic fallback to separately billed API.

## Spike exit criteria
Provider owner verifies official terms, actual sign-in with test account, model availability, rate-limit errors, cancellation, revoke, no token logs and correct usage category. App handoff preserves prompt without URL leak; manual fallback explicitly labeled. Test provider unavailability and quota exhaustion. Require human review before enabling desktop write actions.
