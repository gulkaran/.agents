# Orchestration defaults

These settings apply to orchestrate and its workers. Grilling stays in the
coordinating conversation on the session model.

```yaml
orchestration:
  models:
    investigate: {model: gpt-6-astra, reasoning_effort: medium}
    design: {model: gpt-6-astra, reasoning_effort: medium}
    execute: {model: gpt-6-astra, reasoning_effort: medium}
    pr-review: {model: gpt-6-astra, reasoning_effort: medium}
    review-fix: {model: gpt-6-astra, reasoning_effort: medium}
  max_fix_rounds: 3
  max_replans: 2
  max_nested_agents: 20
```

The execute model applies to the execute coordinator and each PR worker.
Scouts use the shared scout configuration unless explicitly overridden.

A fix round is an approved review-fix pass followed by review. A replan
revisits investigation or design after drift or failed feasibility. The
helper allowance counts scouts, PR workers, and blind reviewers across the run.

Apply overrides by field: this file, optional gitignored `defaults.local.md`,
then the human's request. Preserve unspecified values. Save resolved settings
in topic state and reuse them on resume unless the human changes them.

These settings do not alter the three human approvals or the pause after the
full PR stack. A budget or model choice cannot delegate a human decision.
