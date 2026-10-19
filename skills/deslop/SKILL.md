---
name: deslop
description: Strip a branch of everything that exists for the agent rather than the repo before it becomes a PR: local-testing docs, committed screenshots and previews, probe scripts, debug output, change-narrating comments, and reflexive code patterns. Use after execute's verification passes and before opening-a-pr, or when asked to "deslop" or "clean this branch".
---

# Deslop

An agent leaves two kinds of slop on a branch: artifacts it made to convince itself (guides, previews, screenshots, probe scripts) and code it writes by reflex (comments, defensive wrapping, casts, nesting). Both go before a reviewer sees the diff. Behavior stays the same.

## 1. List what the branch added

```bash
git diff --name-status <base>...HEAD
```

For stacked PRs, use this PR's parent branch as `<base>`.

Then read the repo's `AGENTS.md`, `CLAUDE.md`, or `CONTRIBUTING.md` for doc rules. A table mapping code paths to intent docs, or a rule naming a doc that must change with the code, is an explicit rule. Nothing else is.

## 2. Delete artifacts

Delete from the branch, not only from the PR description:

| Added file | Verdict |
|---|---|
| Doc an explicit repo rule requires for a code path you touched | Keep; strip local-testing references from it |
| Maintained personal app skill under `.agents/skills/verify-*/`, including its feature map and reusable helpers | Keep; work profiles belong in external storage |
| Guide, reference, README, qualification checklist, or "previews" doc you wrote | Delete |
| PNG, GIF, HTML preview, screenshot | Delete |
| Probe script, manual-run fixture, debug output, scratch harness | Delete |
| Test that exercises your local setup rather than a behavior in the design | Delete |

Evidence a human may want later goes to the caller's resolved verification directory; otherwise use `../_shared/verification.md`. Never put it in the application repo. When a deleted file was linked from a doc that stays, remove the link.

Example: the repo's `AGENTS.md` maps the notifier code to `docs/intent/release-pr-notifications.md`. That doc stays and loses its local-testing paragraph. `docs/conveyor/guides/qualifying-release-pr-notifications.md`, `docs/conveyor/reference/release-pr-notification-previews.md`, and the three PNGs beside it are deleted, however well written.

## 3. Remove code slop

Read the remaining diff hunk by hunk against the surrounding file's style:

- Comments that narrate the change, restate the code, or mark the agent's process ("tests-first job contract"). Keep a comment that explains a non-obvious why.
- Defensive checks, rescue or try blocks, and nil guards on trusted internal paths the file does not otherwise guard.
- Casts, `any`, `T.untyped`, or lint disables added to get past an error instead of fixing it.
- Indirection for the agent's convenience: a helper wrapping one call, a constant looked up by built string, a test helper used by one test.
- Nesting an early return would flatten.
- Prose in docs that stay: apply `~/.agents/skills/unslop/SKILL.md`.

Keep behavior unchanged. Put a clear bug fix in its own commit and record it in `decisions.md`. Small edits, not rewrites: restructuring is `code-simplification`'s job, invoked only by the human.

## 4. Verify and commit

Start from committed, passing implementation. Run the tests the branch touches after cleanup. Commit cleanup separately, in the repo's convention, naming what was removed:

```
chore: remove local-testing docs and verification artifacts
```

Never amend, fixup, or squash cleanup into implementation during PR preparation or review. If nothing changes, report a no-op without an empty commit. Include the cleanup commit in the PR handoff.

Report in one to three sentences: what was deleted, what stayed because a rule required it, and any behavior change.

## When not to use

- `code-simplification` restructures code that is already clean of slop.
- `review-fix` addresses findings a reviewer raised.
