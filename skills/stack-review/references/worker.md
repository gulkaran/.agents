# Stack review worker

You review one PR in a stack. Another agent combines your report with the
reports for the other PRs. You get the stack from bottom to top and the PR
that you review.

## 1. Run pr-review

Read `~/.agents/skills/pr-review/SKILL.md` and follow it for your PR with
`--stateless`. You cannot load it as a skill, so read the file. Its
references are in `~/.agents/skills/pr-review/references/`.

- **Range.** Review your PR against its own base commit, not against `main`.
- **Checkout.** Read code with `git diff <base>...<head>` and
  `git show <commit>:<path>`. Never run `git checkout`, `git switch`, or
  another command that changes the checkout, because other workers use it.
  Give the blind reviewer the same rule.
- **Files.** Write no files.

## 2. Check findings against the top of the stack

Skip this step if your PR is the top PR.

For each finding, read the same code at the head commit of the top PR. If the
problem is gone there, remove the finding and add one line:

```
Fixed upstack in PR <n>: <finding title>
```

To find `<n>`, run `git log --oneline <your head>..<top head> -- <path>`.
Match the commit that fixes the problem to the PR whose base and head contain
it. If you cannot tell whether the problem is gone, keep the finding.

## 3. Return the report

Return only this:

```
PR <number>: <title>
Base: <base commit>  Head: <head commit>

<pr-review report, with step 3 annotations if pr-review ran its context pass>

Verification: <commands you ran and what they showed>
Fixed upstack: <one line per removed finding, or "none">
```
