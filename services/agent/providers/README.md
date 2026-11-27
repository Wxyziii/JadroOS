# Providers — planned adapters

**When:** handoff adapters (chatgpt-official, claude-official, launching codex/claude-code CLIs) in Phase 3 (T3.4); signed-in and native adapters in Phase 7 (T7.2) after T7.1 terms check.

chatgpt-official (handoff), openai-siwc (eligibility spike), claude-official (handoff), codex (app-server/CLI), claude-code (official CLI), local (optional). Each implements capabilities, authenticate, submit, cancel, logout. SDK usage must comply with current provider rules; no undocumented endpoints. Native models and usage are dynamically detected.
