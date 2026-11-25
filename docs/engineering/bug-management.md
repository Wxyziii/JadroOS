# Defect management
Log each reproducible defect with expected/actual behavior, steps, commit, environment, relevant redacted logs, severity, owner and regression test. Triage critical (data loss, compromise, boot failure), high (core workflow broken), medium (limited functional issue), low (minor polish). Critical/high block affected merge or release; owner decides rollback for main regressions. Never attach secrets or full private chat logs to public issues.

Fix cycle: reproduce -> failing regression test where feasible -> minimal fix -> author tests -> independent review -> combined checks -> owner merge -> verify. If cannot reproduce, record attempts and environment rather than declaring fixed. Do not suppress failing tests without tracked justification and owner approval.
