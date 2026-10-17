---
name: orchestrate
description: Coordinate grilling, investigation, design, and a complete PR stack with human approvals and fresh workers. Use for "orchestrate" or "run the whole workflow". Pause after the stack exists before continuing to review.
---

# Orchestrate

Coordinate the workflow while the human owns intent, approach, and design
decisions. Save reports and handoffs so a fresh worker can continue without
the conversation. Never substitute an agent's choice for human approval.

## 1. Resolve the run

Read `../_shared/config.md`, `../_shared/topic.md`,
[defaults](references/defaults.md), and [run state](references/run-state.md).
Resolve the repository and topic, then reconcile saved state before launching
work. Preserve approvals only when their document checksums still match.

Existing research can be reused. A request to skip investigation does not
supply a missing approach choice.

## 2. Establish the brief

Run `../grill-me/SKILL.md` in the coordinating conversation. Save the brief
and get the human's confirmation before investigation. Reuse an already
confirmed, unchanged brief.

The parent owns questions, approval records, topic state, and vault commits.
Workers gather evidence and prepare proposals.

## 3. Coordinate investigation and design

Launch a fresh worker per phase with no inherited conversation, using
[the worker brief](references/worker-brief.md). Save its report, verify claims,
and present the proposal through `../wait-what/SKILL.md`.

Pause for the human to choose the investigation approach. Then run design,
including its feasibility work, and pause for approval of the technical
contract. Save each decision under the topic protocol.

If evidence invalidates a choice, preserve it and use the revision transition.
Do not answer open product questions for the human. Return unresolved choices
with evidence and a recommendation.

## 4. Build the complete stack

After all three approvals, launch `../execute/SKILL.md` as the execution
coordinator. It launches one fresh worker per PR and owns execution state
until it returns. Only one coordinator writes topic state at a time.

Pass the resolved execute model to its PR workers. Follow the dependency
order without approval pauses between PRs. Stop for material drift, failed
required checks, or a missing capability.

Execute runs deslop on each PR in separate cleanup commits. It never invokes
code simplification.

## 5. Pause after execution

Record status `awaiting-stack-review` after the whole stack exists and required
checks pass. Keep the topic `in-review`. Present ordered PR links, behavior,
verification, deviations, and cleanup commits, then wait.

Even "take this to a reviewed PR" includes this pause. Do not launch review
or fixes before the human continues from it.

## 6. Continue review when requested

Run `pr-review` on each current PR revision. Present findings before
`review-fix`; that skill's human approval still applies unless the human
explicitly authorizes the specific fix plan.

Use fresh review and fix workers within the saved budgets. Recheck changed
revisions. Never drop acceptance criteria to clear review. Stop when required
checks fail, a human decision is needed, or the budget is exhausted.

Record `ready` only when current reviews have no unresolved blockers and
required checks pass. Merging, deployment, and vault publication are separate.

## When not to use

Invoke an individual phase when only that phase is wanted.
