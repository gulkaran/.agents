---
name: execute
description: Build the approved design's PR stack with one fresh worker per PR, tests first, and a draft PR each. Use for "execute the design" or "build the PR stack". Verification, deslop, and review stay separate.
disable-model-invocation: true
---

# Execute

Turn the approved design into a stack of draft PRs. One fresh worker builds
each PR in order, tests first. This skill opens the PRs and reports their
links.

## 1. Read the PR plan

Run `accio topic current --path` and read `design.md` in that folder. If Accio
reports no topic or the file is missing, stop and tell the human to run
`design` first. Read nothing else. The orchestrator makes no implementation
decisions, so it needs no other context.

Take the PR plan from the design: PR IDs, branches, bases, and order. For each
branch, run `gh pr list --head <branch> --json url,headRefOid`. A branch with
an open PR is complete. Record its head and skip it. Start the first worker at
the first branch without one. Never recreate or force-push an existing PR.

## 2. Launch one worker per PR

Launch a fresh agent with no inherited conversation, on the session model.
Give it [the PR worker contract](references/pr-worker.md) as its task, with
its PR ID, branch, base, and the predecessor's head from the previous handoff
or the skipped PR. The first PR has no predecessor.

Run workers one at a time, in dependency order. Never edit the checkout
yourself. If the harness cannot launch an agent, report that and stop. Do not
build the stack in this conversation.

## 3. Check the handoff

When the worker returns, run one check:

```bash
gh pr view <url> --json isDraft,baseRefName,headRefOid
```

The PR must be a draft on the assigned base, at the head the handoff reports.
If it is, print one line and launch the next worker:

```
pr-2 opened: <url>, tests pass
```

If the check fails or the handoff reports a blocker, stop the stack. Do not
retry or launch a replacement worker. A blocker is a related test that cannot
pass, or material drift: a changed acceptance criterion, public contract, PR
boundary, or a design assumption the code disproves. The worker records minor
deviations in `decisions.md` itself, and work continues.

## 4. Report the stack

After the last worker, print the PRs in order with their status:

```
pr-1: <url>, tests pass
pr-2: <url>, tests pass
pr-3: not started, blocked by pr-2
```

Explain a blocker in ASD-STE100 Simplified Technical English: short sentences,
one idea each, with a named actor. For example: "The worker ran the refund
tests. Two tests fail. The design says `Refund#amount` accepts nil. The model
rejects nil." Then stop. Verification, cleanup, review, and marking a PR
ready are the human's next steps.

## When not to use

- `design` when the implementation contract is not approved.
- `verify` and `deslop` to prove and clean a draft before marking it ready.
- `review-fix` for approved fixes to review findings.
