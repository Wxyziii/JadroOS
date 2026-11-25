# Definition of done
A task is Done only when its acceptance criteria are met, code is reviewed and merged by owner, and post-merge checks pass. An author PR with self-tests is Ready for review, not Done.

- [ ] Ticket and acceptance criteria identified; dependencies satisfied.
- [ ] Code implements requested behavior without unrelated rewrites.
- [ ] Errors, bounds, cancellation and negative paths handled.
- [ ] New tests added and relevant existing tests run; failures fixed.
- [ ] Clean build, lint and applicable checks executed with exact exit codes.
- [ ] Security/privacy/accessibility considered; no secrets, privilege shortcuts or silent billing changes.
- [ ] Actual UI evidence for UI changes; comparable performance data where applicable.
- [ ] Docs and contracts updated, compatibility impact explained.
- [ ] Independent review of exact SHA completed; blocking findings resolved.
- [ ] Combined integration and regression checks run on latest base.
- [ ] Owner approval, merge and post-merge smoke test recorded.

Do not describe unexecuted tests as passing. Document known limitations and environment constraints. Test counts are not a correctness guarantee.
