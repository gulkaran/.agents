# Run state and resume

## The `orchestration` block in `_state.md`

```yaml
orchestration:
  status: running          # running | ready | stopped
  next_phase: design       # investigate | design | execute | pr-review | review-fix
  settings:                # resolved from defaults.md, defaults.local.md, the request
    models: {investigate: {model: gpt-6-astra, reasoning_effort: medium}, ...}
    max_fix_rounds: 3
    max_replans: 2
    max_nested_agents: 20
  fix_rounds_used: 0
  replans_used: 0
  nested_agents_used: 0
  active_agent:            # ID while a worker runs, empty otherwise
```

Start counters at zero only for a new run. Commit every counter change before launching work. Reuse saved settings on resume unless the user asks for a change.

## Per launch

Before: save `next_phase`, the round or attempt, the starting revisions, and the agent ID in `active_agent`; commit. After: clear `active_agent`, record the outcome and the next phase, commit vault changes, then launch the next worker.

## Counting

- A fix round is one `review-fix` run followed by `pr-review`. Three rounds allow an initial review plus three follow-up reviews.
- Fix rounds and replans count across the whole topic, including replans and resumed sessions.
- Nested helpers count across the run, not per phase.

## Resume

1. Inspect `active_agent`, the artifacts, the branch, and any PR before continuing. Reuse completed work and existing PRs.
2. Keep counters and approvals that still apply.
3. Never launch a second writer while the recorded agent may still be running.
4. An interrupted fix: reconcile its commits, review outcomes, and summary entry. Finish it inside the already-counted round, then run its review. An exhausted fix budget blocks a new round, not completion of the active one.
5. Check whether a summary entry already exists before appending it again.
