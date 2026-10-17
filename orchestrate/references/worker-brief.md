# Worker brief

Pass enough approved context for a fresh worker to perform one phase.

## Input

- The phase skill, this file, and the shared topic protocol.
- Repository and topic paths, assigned scope, and resolved model settings.
- The confirmed brief and relevant investigation, design, decisions, and evidence.
- Approval checksums and the human's actual choices.
- Attempt number and the remaining helper allowance.
- For review or fixes, PR identity, base and head, and its latest review round.

Launch with no inherited conversation. Tell the worker to perform only its
phase and return without starting the next phase.

## Phase ownership

Investigation and design workers gather evidence, write draft artifacts, and
return `awaiting-human` with the proposal and unresolved choices. They do not
ask another agent to approve, record approval themselves, or advance state.
The parent presents the proposal and records the human's response.

The execute worker is a coordinator. It verifies approvals and follows its PR
worker contract. While it runs, it owns execution state and vault commits.
The orchestrator waits for its return before editing that state. PR workers
make application commits and write handoffs without committing the vault.

Review and fix workers write their assigned reports without committing the
vault. A fix worker returns its proposed plan before making changes unless
the parent supplies the human's approval of that plan. App commits remain
separate by purpose.

## Report

Return the outcome, proposal or result, artifact paths, unresolved questions,
verification status, and the next required action. Include PR identities and
revisions when applicable. Distinguish human choices from recommendations.

The parent appends the report to `orchestration-summary.md` with phase,
attempt, date, and artifact links, then presents the decision-relevant content
in chat. The summary does not replace the brief, investigation, design, or
per-PR execution handoffs.

## Pauses

Valid outcomes include `awaiting-human`, `blocked`, `complete`, and
`stack-created`. A worker completion does not imply approval. Material drift
returns to the parent with evidence and options.

After `stack-created`, the parent presents the full stack and waits for the
human before launching review. No worker may bypass that pause.
