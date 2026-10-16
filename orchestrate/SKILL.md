---
name: orchestrate
description: Take an issue or task through investigate, design, execute, pr-review, and a bounded review-fix loop with a fresh agent per phase, no human gates, and phase reports saved in the vault. Use when the user says "orchestrate", "run the whole workflow", or "take this to a reviewed PR". Returns the reviewed PR.
---

# Orchestrate

Run the pipeline end to end with the human's approvals delegated to the run. The parent agent owns state, handoffs, and vault commits; one fresh worker per phase owns the research, code, tests, and report. Budgets bound the loop so a stuck review never runs forever.

Invoke with `$orchestrate <issue or task>`. Add `skip investigation` when research exists. Model overrides are optional: `use Astra for execute`.

## 1. Set up the run

1. Read `../_shared/config.md` (plus `config.local.md`), `../_shared/topic.md`, and [defaults](references/defaults.md), which says how overrides resolve.
2. Resolve repository and topic. Fetch linked issues and comments with the tracker recipe. Reuse a matching topic or create one on a `<category>/<slug>` branch.
3. Read `_state.md` before writing it. Log scope, delegated approval, resolved settings, and any investigation skip, and add the `orchestration` block from [run state](references/run-state.md). On resume, follow that file's resume rules before launching anything.

## 2. Delegate each phase

- One fresh agent per phase, including every review and fix round, on the phase's model and reasoning effort, with no parent transcript (`fork_turns: none` where available).
- Brief it per [worker brief](references/worker-brief.md). It runs only its phase and returns a handoff or a replan request, never the next phase.
- Save launch state per [run state](references/run-state.md) before and after every worker, and commit vault changes between phases. Phases run one at a time; never two writers at once.
- Nested helpers (scouts, the blind reviewer) count against `max_nested_agents`. Record each before launch with an explicit model, defaulting to the phase model.
- A missing tool or model is a reported limitation, never a silent substitute or a phase done in the parent.

## 3. Decide in the human's place

- Pick the best-supported approach and reasonable answers to open design questions. Record assumptions and alternatives in the normal artifacts as delegated choices.
- Approve the design only after its feasibility evidence is saved: a passing spike, or the design skill's recorded reason that none was needed. Set `design_approved` and note the approval came from this run.
- Failed feasibility or a material design change: preserve evidence, clear approval, increment `replans_used`, use the reopening or revision transition, and rerun with a fresh agent. Stop with the evidence when `max_replans` would be exceeded.
- Never drop acceptance criteria or relabel a defect as a trade-off to clear review. Contract changes need a recorded reason and the same design and review checks.

## 4. Run the sequence

1. `investigate`, unless skipped. Its agent records the chosen approach and sets `designing`.
2. `design`. On a skip, set `designing` and hand it the issue and supplied research.
3. `execute`. It deslops the branch and opens PRs through `opening-a-pr` itself.
4. `pr-review` on every PR. Its `review/round-<n>.md` records base and head revisions, unresolved Must fix IDs, check results, and gaps.
5. Blockers left and budget left: increment `fix_rounds_used`, run `review-fix` with its plan saved but not gated, then a fresh `pr-review` that rechecks rejected findings too. Repeat. A finding only a human can close stays open; a Must fix of that kind stops the run.
6. Finish when the latest review covers the current revisions, finds no unresolved blocker, and required checks pass. Optional improvements go in the report, not another round.

Failing required checks are blockers. A PR changed after its review is unreviewed until reviewed again. Budget exhausted with blockers left: `status: stopped`.

## 5. Preserve each report

Workers read `../technical-writing/SKILL.md`, then append the report the skill would have given the human to `orchestration-summary.md` in the worker brief's format. Earlier entries stay; corrections are appended. The parent chat gets brief progress, never pasted reports or approval requests.

## 6. Return the result

Set status `ready` or `stopped`, log, and commit the vault. The topic stays `in-review` while a PR exists. Return PR links, what works, verification results, fix rounds used, and the summary path. If stopped, name the blockers or the missing capability; an incomplete review is never clean. Merging, deploying, and publishing the vault are outside this workflow.
