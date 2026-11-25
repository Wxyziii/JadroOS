# Ownership and edit boundaries
Ownership is a workflow convention, not an assertion of GitHub permissions or agent identity. Repository owner is final integrator. Both agents implement and validate their work.

| Path | Default author | Independent reviewer |
|---|---|---|
| services/policy, services/search, services/agent, services/system | Codex | Claude |
| platform/** | Codex | Claude |
| desktop/compositor | Codex | Claude |
| desktop/spotlight, desktop/shell, desktop/design-system | Claude | Codex |
| apps/** | Claude (UI), Codex (separate service adapters) | other agent |
| protocols/**, config/defaults.json | owner-approved contract PR, designated author | other agent |
| root CMakeLists.txt, shared build tooling | one designated author per ticket | other agent |
| .github/workflows/backend-ci.yml | Codex | Claude |
| .github/workflows/desktop-ci.yml | Claude | Codex |
| shared integration/security/release workflows | designated author, owner approval | other agent |
| tests/**, docs/** | ticket-specific paths; shared docs one author | other agent |

Each ticket declares exact paths, not just a component. Root CMake and protocol files are conflict hotspots; neither agent edits them without a reservation. Tests live next to component wherever feasible. New files under an owned directory are fine; changes elsewhere require a separate ticket or coordination. Do not reformat unrelated files.

Branches: codex/J-xxx-short-name, claude/J-xxx-short-name, integration/J-xxx. Author pushes own branch only. Reviewers inspect commit using detached worktree and leave comments rather than editing author branch. GitHub CODEOWNERS can point at real humans/teams, not fictional AI accounts. Owner may add actual collaborators later. Do not use self-approval as independent review even if both CLI tools use same GitHub account.
