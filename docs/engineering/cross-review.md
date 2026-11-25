# Independent cross-review playbook
Reviewer must not simply accept author summary. Inspect actual diff, relevant documentation, tests and call sites. Review exact commit SHA and base; reproduce checks in a separate checkout with clean build output.

1. Read ticket, contracts, acceptance and author evidence.
2. Inspect boundary conditions, unexpected input, authorization, error paths, resources, races, timeouts, compatibility and dependencies.
3. Run relevant tests independently; add or suggest a reproducer for missing behavior. Try at least one negative scenario not demonstrated by author.
4. Findings include severity, path:line, impact, reproduction and proposed correction. Mark assumptions and untested hardware.
5. Return APPROVE, REQUEST_CHANGES or BLOCKED with exact SHA, commands, exit codes and limitations. Approval is limited to reviewed revision.
6. Author fixes; reviewer rechecks new SHA. Owner decides merge after integration.

## Review checkout example
From the clean coordinator checkout:
```sh
git fetch origin
git worktree add --detach ../review-codex origin/codex/J-002A-backend-foundation
# Inspect/build/test in ../review-codex; do not run unknown scripts blindly.
git worktree remove ../review-codex
```
Use unique review paths and independent build outputs. Review of agent code is execution of untrusted code: inspect commands and run only in disposable VM, no provider credentials exposed to test process. For suspicious code use fresh credential-free VM.

## Review report
Reviewed SHA; verdict; findings; tests/commands; actual results; coverage limitations; integration impact. If both agents share GitHub credentials, record review as independently produced text, not as a claim of separate GitHub identities or bypass of branch protection.
