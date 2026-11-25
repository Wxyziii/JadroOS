# Spotlight specification

**When:** Phase 3 builds the Spotlight UI in the shell, General search (apps, settings, opt-in files, calculator) and the AI tabs as *handoffs* to the official apps and CLIs. Phase 7 adds native answers inside Spotlight, the Medium/High/Extra High modes and Local AI. See the [roadmap](../planning/roadmap.md).
Confirmed: General default, AI tabs, official app where possible. Alt+Tab remains system window switching. Proposed Alt+Space opens Spotlight; Ctrl+Tab and Ctrl+1..6 switch tabs; shortcuts configurable. Do not intercept Alt+Tab inside Spotlight.

## Tabs and routing
| Tab | Default | Handoff or backend |
|---|---|---|
| General | local search | apps, settings, files, clipboard opt-in, calculations, typed actions |
| ChatGPT | Instant | official ChatGPT web/native client; if no documented prompt handoff, open and let user paste explicitly |
| Claude | chat | official Claude web/native app, same handoff rule |
| Codex | agent | official CLI/app-server, user session and sandbox |
| Claude Code | agent | official CLI in terminal; SDK only supported auth |
| Local AI | local | optional llama.cpp/Ollama, explicit install |

ChatGPT tab offers Instant, Medium, High, Extra High; native inference modes are available only after capability/eligibility detection. UI modes do not promise a provider model or identical quality. Never route Instant to SIWC/API/Codex automatically. Offer official ChatGPT for existing conversations/memory/projects. A native mode is a separate conversation history. No unsupported chat submission, DOM scraping or session cookie reuse.

## State machine
Closed -> Open(General, empty) -> Typing -> Results or ProviderReady -> Submitted -> Handoff/Running -> Success/Failure/Cancelled -> Closed. New invocation defaults General; explicit deep-link may select provider after action. Preserve failed prompt locally until user dismisses. Provider tab switch preserves draft per tab only during open session; no disk persistence unless user opts in.

## Ranking and interaction
Exact app/name first, prefix, fuzzy, recent local actions, indexed file results; stale queries canceled via monotonically increasing generation ID. Default no home-wide indexing: opt-in directories, no secrets/cache by default. Escape closes, arrows move selection, Enter activates selected result, Ctrl+Enter optional explicit submit. Commands displayed as structured actions; terminal execution separate, never eval search text. Clipboard and screenshot attachments require explicit selection and preview.

## UI sketch (structural, not visual reference)
```text
[ Search or ask ...                               ]
[ General | ChatGPT | Claude | Codex | Claude Code | Local ]
[ Instant v ] (only ChatGPT)
[ Results / prompt preview / usage notice         ]
[ Open app / submit / cancel                      ]
```

## Handoff UX
Open official provider URL using trusted constant or verified app handler. If official external prompt API is unavailable, show Copy prompt button and Open app; avoid forced paste, fake keystrokes or automatic sending. If native app unavailable, open official web fallback after showing target. No silent paid fallback. Show provider unavailable, disconnected, model unavailable, rate limit and consent denied states. Preserve Unicode and multiline prompts. Do not include user prompt in URL, process arguments, logs or telemetry. For Codex/Claude Code use explicit working directory and reviewed sandbox; no command injection.

## Acceptance
Fresh open always General. Alt+Tab retains OS MRU. Selecting ChatGPT defaults Instant. Instant routes only official chat. Medium/high modes require detected native capability or explicit user selection in official UI. Claude and Codex are independent. All tabs keyboard operable, screen-reader labels present. No network call from typing or General. Provider failure does not auto-bill. The current `desktop/spotlight` prototype only renders UI and emits a local status; it is replaced by the shell overlay in T3.3.
