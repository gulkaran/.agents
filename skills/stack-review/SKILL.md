---
name: stack-review
description: Review a stack of PRs with one pr-review worker per PR, all in parallel, and give one report. Removes findings that repeat upstack and findings that a later PR fixes. Use only when the human asks to review a stack, for example "/stack-review <links>" or "review this stack". Pass --stateless to write no files.
---

# Stack review

Review each PR in a stack with `pr-review`, then give one report for the
stack. A defect appears one time, on the lowest PR that adds it. A finding
that a later PR fixes is not a finding, because the stack merges together.

## 1. Order the stack

Work in a clone of the PRs' repository. If the current directory is not one,
stop and ask. Get each PR that the human names:

```sh
gh pr view <n> --json number,title,url,author,baseRefName,headRefName,baseRefOid,headRefOid
gh api user --jq .login
```

- **Order.** The base of the bottom PR is not the head of another PR. The
  base of each next PR is the head branch of the PR below it. If the bases do
  not make one chain, stop and ask.
- **Owner.** Mark each PR as yours or a coworker's. Step 4 saves rounds only
  for yours.
- **Fetch.** Run `git fetch origin <bottom base> <head branches>`. Workers
  read code by commit and never check out a branch, because they share one
  checkout.

## 2. Launch one worker per PR

Launch all workers at the same time. Each worker is a fresh agent on the
session model, with no inherited conversation. Give it
[the worker contract](references/worker.md) as its task, with:

- the stack from bottom to top: number, title, base commit, and head commit
  of each PR,
- the PR that it reviews.

Workers always run `pr-review` with `--stateless`. Only this skill saves
rounds, so two workers never get the same round number.

If a worker fails, report its PR as not reviewed. Do not review it yourself.

## 3. Remove repeated findings

- **Repeats.** An upstack finding with the same cause as a downstack finding
  stays only on the lower PR. Example: PR 1 adds `parse_date` with an
  off-by-one bug. PR 2 calls `parse_date`, and its worker reports the same
  bug. Keep the finding on PR 1 only.
- **Numbers.** Number all findings in one sequence, from the bottom PR up, so
  the human can say "fix 2 and 7". Update each verdict to the new numbers.

## 4. Report and save

Print one section per PR, from bottom to top. Each section has the PR number
and title as its heading, the worker's report in the `pr-review` format, and
its `Fixed upstack in PR <n>: <title>` lines.

Without `--stateless`, save a round for each of your PRs, from bottom to top:

1. Run `accio path review --next` and write the round at the printed path:
   base and head, findings with annotations, verification, and what was not
   checked. Write each file before you run the next command.
2. After the last round, run `accio commit -m "add stack review rounds"`.

If Accio reports no topic, say that no rounds were saved.

Post nothing to GitHub unless the human asks.

## When not to use

- One PR: `pr-review`.
- Fixing findings: `review-fix`.
