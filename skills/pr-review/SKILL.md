---
name: pr-review
description: Review a PR for bugs, risk, missing tests, performance, and complexity. Use for "review this PR", "review my PR", or "review the stack". A blind code review precedes checking your PR against its approved design. Changes no code.
disable-model-invocation: true
---

# PR review

Review correctness, security, scale, tests, performance, and unnecessary code.
Keep the initial reviewer unaware of the design so it evaluates the code
independently. Analysis only; never edit code or tests.

Read first: `~/.agents/skills/_shared/config.md`.

## 1. Scope the PR

- Target: the PR number, URL, or branch the human names. Otherwise match the
  current branch to the topic's `prs` entries, or use its legacy `pr` record.
  If several PRs remain possible, ask which to review. An explicit stack review
  covers each PR separately against its own base.
- Record base, head, and whether uncommitted changes are included, so every
  finding has an unambiguous range.
- Decide whose PR it is. Yours: run steps 2, 3, and 4. A coworker's: steps 2
  and 4.

```sh
gh pr view <n> --json author --jq .author.login
gh api user --jq .login
```

## 2. Blind review

Launch one fresh subagent on the session model with the diff range, PR
description, and `references/blind-review.md`. That reference defines its
inspection and report. Give it no topic documents.

## 3. Context pass (your PR only)

Resolve the topic and read `design.md` and `decisions.md`. Then:

1. **Annotate each blind finding** as `stands` or `conflicts with a decision`,
   citing the entry. A finding that argues against a recorded human choice is
   still reported; the human decides whether the choice holds. A decision
   never covers a defect separate from the trade-off it accepted.
2. **Add design findings.** Code that violates an acceptance criterion, a
   design assumption, or a recorded decision is a finding; cite the line it
   violates.
3. **Check test integrity.** Use `execution/<pr-id>.md` to identify red and
   green cycles. Compare each behavior's test with its red version. Later
   tests introduced in their own red cycles are expected. Changed earlier
   tests need justification in `decisions.md`. Missing history is a gap,
   never a claimed pass.
4. **Check cleanup separation.** Verify deslop ran on this PR's changes and
   that cleanup commits follow committed implementation. Any requested
   simplification must have its own commits. A deslop no-op needs no commit.

Deviations the executing agent recorded are claims to verify, not permission.
Every fix you propose is a code change, a deletion, or a question for the
human. A missing record, capture, or document is a gap to report, never a
finding to close by creating one.

## 4. Report

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before reporting.
Use the format in `references/blind-review.md`, with step 3 annotations on
their findings.

- Your PR: write the next `review/round-<n>.md` in the topic folder: base
  and head, findings with annotations, verification, and what you could not
  check. One reviewed revision per file; never edit an earlier round or
  carry its history forward. Log one line in `_state.md` and commit under
  the topic protocol. Recommend `review-fix` for actionable findings.
- A coworker's PR: report in chat. Post nothing to GitHub unless asked.

## When not to use

- Fixing findings: `review-fix`.
- Simplifying working code without a review: `code-simplification`.
