"""Offline structural checks, not a build or security test."""
from pathlib import Path
import json
import re

root = Path(__file__).resolve().parents[1]
required = [
    "README.md", "AGENTS.md", "SECURITY.md", "docs/product/requirements.md",
    "docs/architecture/overview.md", "docs/design/references.md",
    "docs/design/spotlight.md", "docs/security/threat-model.md",
    "docs/planning/roadmap.md", "docs/planning/task-board.md",
    "docs/engineering/workflow.md", "docs/decisions/0005-quickshell-shell.md",
    "docs/decisions/0006-phased-build-order.md", "docs/decisions/0007-language-policy.md",
    "CLAUDE.md", "services/policy/src/lib.rs",
    "desktop/spotlight/Main.qml", "protocols/agent-request.v1.schema.json",
]
errors = [f"Missing {p}" for p in required if not (root / p).is_file()]
for path in root.rglob("*.md"):
    if any(p in {".git", "build", "target"} for p in path.parts):
        continue
    for dest in re.findall(r"\]\(([^\s)]+)\)", path.read_text(encoding="utf-8")):
        if dest.startswith(("http://", "https://", "#", "mailto:")):
            continue
        if not (path.parent / dest.split("#", 1)[0]).exists():
            errors.append(f"Broken relative link {path.relative_to(root)}: {dest}")
defaults = json.loads((root / "config/defaults.json").read_text())
assert defaults["desktop"]["spotlight_default_tab"] == "General"
assert defaults["ai"]["instant_route"] == "official-chatgpt"
assert defaults["ai"]["automatic_paid_fallback"] is False
assert defaults["ai"]["admin_operations_enabled"] is False
json.loads((root / "protocols/agent-request.v1.schema.json").read_text())
if errors:
    raise SystemExit("\n".join(errors))
print("Repository structural checks passed")
