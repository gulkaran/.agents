---
name: execute
description: Implement an approved design with one fresh worker per PR, behavior tests first, and deslop in separate commits. Use for "execute the design" or "build the PR stack". Pause after the full stack is created.
disable-model-invocation: true
---

# Execute

Coordinate one fresh implementation worker per PR in the approved design.
Build the complete stack without approval pauses between PRs, then hand it to
the human. Material drift interrupts execution before the affected work proceeds.

## 1. Check the contract

Run `accio topic current --path` and read `investigation.md`, `design.md`,
`decisions.md`, and `feasibility/` in that folder. If Accio reports no topic,
stop and tell the user to run investigate first. The PR plan in `design.md` is
the contract.

Before resuming, compare that PR plan with existing branches and open PRs.
Do not recreate a completed PR or overwrite an active worker's changes.

## 2. Launch a PR worker

Read [the PR worker contract](references/pr-worker.md) and supply its inputs.
Supply a distinct evidence directory per PR using
[the verification protocol](../_shared/verification.md). The verifier discovers
the app skill at verification time. Launch a fresh agent with no inherited
conversation per PR.

Use the session model unless the user or orchestrator specifies another.
Run workers sequentially in dependency order, with one application writer.
Fresh context does not require a new worktree. Reconcile a worker's branch and
PR before another writer starts.

A missing delegation capability is a reported blocker; do not silently build
the whole stack in the coordinator.

## 3. Handle the handoff

Verify the worker's branch, base, commits, PR URL, and reported checks against
the handoff it returned. When it reports deviations, append them to
`decisions.md` and commit with `accio commit -m "record <pr-id>"`. Pass the
verified handoff to workers that depend on it.

Check that the worker followed the contract's sequence: TDD, committed green
implementation, separate deslop cleanup, fresh verification of the final
revision, then PR creation with inspected evidence. Never launch code
simplification automatically.

Material drift is a changed acceptance criterion, public contract, dependency,
storage or migration plan, PR boundary, or a disproven design assumption.
Return it to the human with its consequence and options. A failed required
check or unavailable verification blocks claiming that PR complete. Report the
gap without manufacturing evidence.

## 4. Verify the stack

After every planned PR exists, compare the stack against the design's scopes
and requirement IDs. Check dependency bases, current heads, verification
results, and separate cleanup commits. A child PR's diff must not include its
parent's feature as new work.

Reuse passing checks for unchanged revisions. Run affected checks again after
repairs. Do not call the stack ready while a required check is failing or a
planned capability is missing.

## 5. Pause for the human

Use `../wait-what/SKILL.md` to report PR links in order, delivered behavior,
verification, deviations, test edits, and cleanup commit links or deslop no-ops.

Pause here, even under orchestrate. Do not start review, simplification, merge,
or deployment until the human continues. The pause happens after the complete
stack, unless drift or a blocker required an earlier stop.

## When not to use

Use `design` when the implementation contract lacks approval.
Use `review-fix` for approved fixes to review findings.
