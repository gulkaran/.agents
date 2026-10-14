# Orchestration defaults

These settings apply only to `orchestrate` and the agents it launches.
Edit each phase's model and reasoning effort here.

```yaml
orchestration:
  models:
    investigate: {model: gpt-6-astra, reasoning_effort: medium}
    design: {model: gpt-6-astra, reasoning_effort: medium}
    execute: {model: gpt-6-astra, reasoning_effort: medium}
    self-review: {model: gpt-6-astra, reasoning_effort: medium}
    review-fix: {model: gpt-6-astra, reasoning_effort: medium}
  max_fix_rounds: 3
  max_replans: 2
  max_nested_agents: 0
```

- A fix round is one `review-fix` run, followed by `self-review`.
- A replan revisits investigation or design after failed feasibility or design drift.
- `max_nested_agents` limits the total helpers across the run, not helpers per phase.
- Per-machine overrides belong in `defaults.local.md` beside this file. That file
  is optional and gitignored. Use the same YAML structure with only the changed fields.
- Apply overrides by field. Use this file, then `defaults.local.md`, then the
  user's request. Keep unspecified values.
- Save resolved settings in the topic's `_state.md`. Reuse them when resuming
  unless the user requests a change.
