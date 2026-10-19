# Run state and resume

Store the orchestration record in the topic's state. Topic approvals remain
under the shared protocol; the orchestration record cannot supply them.

## Run record

```yaml
orchestration:
  status: running # awaiting-human | awaiting-stack-review | ready | stopped
  next_phase: grilling # investigate | design | execute | pr-review | review-fix
  pending_decision: <brief, approach, design, drift, stack, or fix plan>
  settings:
    models: <resolved phase models and reasoning>
    max_fix_rounds: 3
    max_replans: 2
    max_nested_agents: 20
  fix_rounds_used: 0
  replans_used: 0
  nested_agents_used: 0
  active_agent: <worker ID while running>
  active_phase: <phase while running>
```

Initialize counters only for a new run. Preserve unrelated fields and historical
approvals from existing topics. An old delegated approval cannot pass a new
human checkpoint. Follow the shared protocol's existing-topic rules.

## Launch and return

Before launch, save the phase, attempt, starting revisions, and intended scope.
After launch, save the worker ID before further coordination. Run one phase
at a time. Missing capability or model is a reported limitation, never a
silent substitute.

Workers return evidence and questions. The parent saves and commits their
artifacts, updates state, and sets `awaiting-human` when a decision is needed.
Only the actual human response clears the pending decision.

During execute, its coordinator owns state and vault commits. It records
each active PR and worker under `execution`, then clears the worker after
its handoff is reconciled. It returns helper usage to the parent.

After execution completes, set `awaiting-stack-review` and `pending_decision:
stack`. Leave `next_phase: pr-review`. Do not launch review until the human
responds after seeing the stack, even if the original request included review.

## Budgets

A fix round is one approved fix pass followed by a fresh review. A replan
revisits investigation or design after invalidating evidence. Count attempts
across resumes. Increment only when starting the relevant work, not when
waiting for approval.

The helper allowance includes scouts, PR workers, and blind reviewers.
The execute coordinator is the phase worker, not an additional nested helper.
Reserve a slot before launching each helper. A budget limits work; it never
permits skipping approval or combining PRs to avoid fresh workers.

## Resume

Reconcile active agents, artifact revisions, branches, handoffs, and existing
PRs before launching a replacement. Never start another writer while the old
worker may still run. If the worker no longer exists, preserve partial work
and pass it to a fresh replacement.

Validate approval checksums. A pending decision stays pending until the human
answers it. Reuse completed work and existing PRs. Avoid duplicate summary
entries by checking phase, attempt, and revision.

An interrupted fix can finish inside its already-counted round, followed by
review. Exhaustion blocks another round, not completion of the active one.
A changed PR base or head needs a fresh review before it can be reported ready.
