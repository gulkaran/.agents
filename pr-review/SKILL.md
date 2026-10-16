---
name: pr-review
description: Review a pull request for bugs, risk, scale, missing tests, speed, and lean code. Works on any PR by number, URL, or branch, or on the current topic. A blind pass reviews the code with no knowledge of the design; when the PR is yours, a second pass checks each finding and the code against the topic's design and decisions. Writes review.md for your own PRs; changes no code. Use for "review this PR", "review my PR", or after execute.
disable-model-invocation: true
---

# PR review

Review like the engineer who gets paged when it breaks. Order: correct, safe,
holds under load, tested, fast, lean. The reviewer who reads the code must
not know the design: a design is a reason to be charitable, and charity hides
bugs. Analysis only; never edit code or tests.

Read first: `~/.agents/skills/_shared/config.md`.

## 1. Scope the PR

- Target: the PR number, URL, or branch the human names. Otherwise the
  current topic's PR from `_state.md`, or the current branch.
- Record base, head, and whether uncommitted changes are included, so every
  finding has an unambiguous range.
- Decide whose PR it is. Yours: run steps 2, 3, and 4. A coworker's: steps 2
  and 4.

```sh
gh pr view <n> --json author --jq .author.login
gh api user --jq .login
```

## 2. Blind review

Launch one fresh subagent on the session model. Give it the diff range, the
PR description, and `references/blind-review.md`. Give it nothing from the
topic folder: no `design.md`, `decisions.md`, or `investigation.md`. It
reads the diff, the touched files, callers and callees, and the tests, and
returns numbered findings in that file's format.

Scouts may collect callers and usages for it, following
`~/.agents/skills/_shared/scout.md`.

## 3. Context pass (your PR only)

Resolve the topic and read `design.md` and `decisions.md`. Then:

1. **Annotate each blind finding** as `stands` or `conflicts with a decision`,
   citing the entry. A finding that argues against a recorded human choice is
   still reported; the human decides whether the choice holds. A decision
   never covers a defect separate from the trade-off it accepted.
2. **Add design findings.** Code that violates an acceptance criterion, a
   design assumption, or a recorded decision is a finding; cite the line it
   violates.
3. **Check test integrity.** Diff every touched test file against its slice's
   red commit. A post-red change with no justification in `decisions.md` is a
   finding. No red history: record that instead of claiming the check passed.

Deviations the executing agent recorded are claims to verify, not permission.

## 4. Report

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before
reporting. Use the output format in `references/blind-review.md`: what the
change does, findings in Must fix, Should fix, and Nice to have with stable
numbers, a verdict, and what you could not check. Step 3 annotations appear
on their findings.

- Your PR: write `review.md` in the topic folder with findings, annotations,
  scope, and verification. Log one line in `_state.md` and commit under the
  topic protocol. Recommend `review-fix` for actionable findings.
- A coworker's PR: report in chat. Post nothing to GitHub unless asked.

## When not to use

- Fixing findings: `review-fix`.
- Simplifying working code without a review: `code-simplification`.
