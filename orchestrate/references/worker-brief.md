# Worker brief

What the parent passes to every phase agent, and what comes back.

## Pass

- The phase skill path, such as `~/.agents/skills/execute/SKILL.md`.
- `~/.agents/skills/orchestrate/SKILL.md`: its delegated approvals replace the phase skill's human gates for this run.
- Topic folder path and repository path.
- Task scope and the delegated approval statement.
- Relevant artifact paths: `investigation.md`, `design.md`, the latest `review/round-<n>.md`.
- Round or attempt number, and for review phases the base and head revisions.
- The phase model and reasoning effort, and the nested-helper allowance left.

Tell the worker:

- Run only this phase. Return instead of launching the next one.
- Write vault artifacts without committing them; the parent commits after the phase.
- Make the application commits the phase requires (execute's red and green commits, deslop's cleanup commit, review-fix's fix commits).
- Record decisions as delegated choices, not human replies.

## Report

Append to `orchestration-summary.md` under a heading with phase, round or attempt, date, and model, followed by artifact links and the reviewed revision where relevant. Include what the skill would have told the human: findings and chosen approach, design reasoning and feasibility, implementation results, review findings, fix plans and outcomes. Keep `investigation.md`, `design.md`, `decisions.md`, and the current review round current; the summary collects reports and does not replace them.

## Return

Outcome, artifact paths, PR links, relevant revisions, blocker IDs, verification status, and the next phase or a concrete failure reason. Nothing else: the report itself goes to `orchestration-summary.md`.
