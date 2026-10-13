---
name: orchestrate
description: Take an issue or task through investigation, design, implementation, and a bounded review-fix loop using fresh phase agents. Save phase reports in the vault and return the reviewed PR.
---

# Orchestrate

Run the workflow end to end when the user invokes this skill or requests full
orchestration. The user delegates approach selection and phase approvals.

Use `$orchestrate <issue or task>`. Add `skip investigation` when research is
already available. Model overrides are optional, such as `use Astra for execute`.

## 1. Set up the run

1. Read `../_shared/config.md`, its local override if present, and `../_shared/topic.md`.
   Load [orchestration defaults](references/defaults.md) only for this workflow.
   Apply its override rules before choosing phase models or loop limits.
2. Resolve the repository and topic from the input. Fetch linked issues and comments
   with the configured tracker recipe. Reuse a matching topic or create one.
3. Read existing `_state.md` before changing it. Save the task's scope, delegated
   approval, resolved orchestration settings, and any investigation skip in its log.
4. Add an `orchestration` field to the state with `status`, `next_phase`, `settings`,
   `fix_rounds_used`, `replans_used`, `nested_agents_used`, and `active_agent`.
   Start counters at zero only for a new run. Use statuses `running`, `ready`, or `stopped`.
5. Use a `<category>/<slug>` branch.

On resume, inspect the recorded agent, artifacts, branch, and PR before continuing.
Reuse completed work and existing PRs. Preserve counters and approvals that still
apply. Do not launch a second writer while the previous agent is running.
Reconcile an interrupted fix with its commits, review outcomes, and summary entry.
Finish incomplete work within that already-counted round, then run its review.
An exhausted fix budget prevents a new fix round, not completion of the active round.

## 2. Delegate each phase

- Launch one fresh agent per phase, including every review and fix round.
- Set its model and reasoning effort from the resolved phase settings. Use a fresh
  context without the parent transcript, such as `fork_turns: none` when available.
- Pass the phase skill path, this skill path, topic path, repository path, task
  scope, delegated approval, and relevant artifact paths. Let the worker read them.
- Tell workers to run only their assigned phase. They return handoffs or replan
  requests to the parent instead of launching the next phase themselves.
- Keep the parent focused on state, handoffs, and checkpoints. Workers own research,
  code, tests, and detailed reports. Do not load their full transcripts into the parent.
- Run phases sequentially. Each worker writes its artifacts and summary, then
  returns a short handoff. The parent commits vault changes before the next phase.
- Before launch, save `next_phase`, the round or attempt, and starting revisions.
  Record the agent ID in `active_agent`. After completion, clear that ID, record
  the outcome, and save the next phase. Commit counter changes before launching work.
- Allow nested agents only within `max_nested_agents`. The parent records each
  helper before launch and assigns its model explicitly. Default to the phase model.
- If a required tool or configured model is unavailable, report the limitation.
  Do not silently substitute a model or perform the phase in the parent.

## 3. Apply delegated approvals

- Choose the best-supported approach and reasonable answers to unresolved design
  questions. Record assumptions, alternatives, and reasons in the normal artifacts.
- Do not ask for approach selection, design approval, execution approval, or fix
  approval. These decisions belong to the agents during this run.
- Save the design and feasibility evidence before approving it. Set `design_approved`
  with the date and record that approval came from this delegated workflow.
- Require a passing feasibility spike. When no assumption needs a spike, accept the
  design skill's recorded explanation. Failed or inconclusive evidence is not a pass.
- If feasibility fails or implementation needs material design changes, preserve the
  evidence, clear approval, and revisit investigation or design with a fresh agent.
  Use the shared reopening or design revision transition to set the phase before launch.
  Increment `replans_used` before each return. Reapprove the revised design before execution.
- Resolve choices within the requested outcome. Do not drop acceptance criteria or
  mark defects as accepted trade-offs just to clear review. Changes to the contract
  need a recorded reason and the same design and review checks.
- If the outcome cannot be delivered within the user's constraints, stop with the
  evidence. Allow at most `max_replans` returns to planning. Complete the last allowed
  attempt, but stop if another replan is needed.

## 4. Run the workflow

1. Run `investigate` unless skipped or already complete. Let its agent select and
   record the recommended approach, then set the topic to `designing`.
2. Run `design`. If investigation was skipped, set the topic to `designing` and use
   the issue and supplied research directly. Record the skip without another confirmation.
3. Run `execute` after design approval. Follow its test, commit, verification, and
   PR rules. Push and open the planned PRs automatically.
4. Run `self-review` on every PR in the topic. Record the base and reviewed head
   revisions, unresolved `Must fix` IDs, check results, and verification gaps in `review.md`.
5. If blockers remain and the fix budget allows, increment `fix_rounds_used`, then
   run `review-fix`. Verify findings and fix valid blockers within the approved design.
   Save its fix plan before implementation without waiting for human approval.
6. Run a fresh `self-review` after every fix round, including the last allowed round.
   Recheck rejected findings too. Repeat steps 5 and 6 while blockers remain.
7. Finish only when the latest review covers the current PR revisions, finds no
   unresolved blockers, and required checks pass. Keep optional improvements in the
   report without extending the loop for them.

Count fix rounds across the whole topic, including replans and resumed sessions.
Three fix rounds allow an initial review plus three follow-up reviews. Required
check failures are blockers. Missing required verification prevents a clean result.
If the limit is reached with blockers left, set `status: stopped` and report them.
If the PR changes after review, review the new revision before declaring it ready.

## 5. Preserve each phase's report

- Have each worker read `../_shared/style.md` immediately before writing its report.
- Append what the skill would normally tell the user to the topic's
  `orchestration-summary.md`. Preserve the explanation and its useful detail.
  Do not replace it with a terse handoff or a raw tool log.
- Give each entry a heading with the phase, round or attempt, and date. Include
  the actual model, links to its artifacts, and the reviewed revision where relevant.
- Include investigation findings and the chosen approach, design reasoning and
  feasibility, implementation results, review findings, and fix plans and outcomes.
- Keep earlier entries. Append a correction or revised report when decisions change.
  On resume, check whether an entry already exists before appending it again.
- Keep `investigation.md`, `design.md`, `decisions.md`, and `review.md` up to date.
  The summary collects the readable reports and does not replace those records.

Each worker returns only its outcome, artifact paths, PR links, relevant revisions,
blocker IDs, verification status, and the next phase or concrete failure reason.
Send routine progress briefly. Do not paste phase reports or request phase approvals
in the parent chat.

## 6. Return the result

1. Set orchestration status to `ready` or `stopped`, update the topic log, and commit
   the vault checkpoint. Keep the topic phase `in-review` if a PR exists.
2. Return the PR links, a short description of what works, verification results,
   fix rounds used, and a link to `orchestration-summary.md`.
3. If stopped, name the remaining blockers or missing capability. Never describe an
   incomplete review as clean. Leave any existing PR available for inspection.

Finish at the reviewed PR. Merging, deploying, and publishing the vault are outside
this workflow.
