---
name: review-fix
description: Verify review findings (from review.md and/or PR review comments), plan the smallest safe fixes, and - after human approval - implement each as its own commit, then post verdicts and resolve the threads on the PR so the review work leaves a trail of evidence. Use after self-review or when a PR has review comments to address.
disable-model-invocation: true
---

# Review-fix

Close the loop on review findings with an auditable trail: every finding gets
a verdict, valid ones get a fix commit, and every PR thread ends with a reply
and a resolution — visible on the PR afterward, not just in a chat log.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.

## Preconditions

Resolve the topic. Load `design.md` and `decisions.md`. Gather findings from
`review.md` and/or the PR's review threads
(`gh api` GraphQL `reviewThreads` for unresolved threads, or the tracker
recipes' equivalent).

## Process

### 1. Verify every finding

Against the actual code, with `decisions.md` in hand:

- **Valid** — real issue; note severity and the smallest safe fix.
- **Invalid** — wrong, already handled, or a recorded deliberate deviation;
  note why, citing the decision or the code.

Never fix an unverified finding.

### 2. Plan and gate

Read `~/.agents/skills/_shared/style.md` now, immediately before reporting.
Explain the verified issues, their impact, and the smallest proposed fixes in
simple terms, blockers first. Use a compact list or table only if it helps;
summarize rejected findings by why no fix is needed. Preserve all verdicts
and evidence in `review.md`. **Wait for approval** of the plan; the
human may drop, defer, or adjust items.

Before waiting, commit the review verdicts and state log under the topic
protocol. These vault checkpoints are separate from application fix commits
and do not approve the proposed fixes.

### 3. Implement

- One commit per fix, message referencing the finding
  (`fix(review): <finding> — <what changed>`).
- Fixes obey the same rules as execute: don't weaken tests; a fix that changes
  behavior beyond its finding is drift — record or raise per the drift policy.
- Run the relevant tests; nothing gets pushed red.

### 4. Leave the trail

After pushing:

- **Valid findings:** reply on the PR thread with the one-line fix summary and
  the fix commit link, then resolve the thread (GraphQL
  `resolveReviewThread`).
- **Invalid findings:** reply with the reasoned verdict — citing the decision
  or code — and resolve.
- Findings that came only from `review.md` (no PR thread) get their outcome
  recorded in `review.md`.

Update `review.md` with all verdicts and outcomes. Log line in `_state.md`;
phase → `done` when the human confirms nothing remains.
Keep retained test results in `verification/` and commit the review outcomes,
decisions, verification artifacts, and state together using the topic protocol.

Reread `~/.agents/skills/_shared/style.md` immediately before the final handoff.
Briefly explain what was fixed, whether verification passed, and anything
still requiring attention.
