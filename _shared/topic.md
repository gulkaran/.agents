# Topic protocol

A topic holds one piece of work from grilling through review. Every phase
reads this protocol and its state before acting. The human confirms the brief,
chooses the approach, and approves the design before implementation starts.

## Topic folder

```
<vault_path>/<project>/topics/<slug>/
  _state.md         # phase, approvals, PRs, and session log
  brief.md          # intent, examples, constraints, and interview answers
  investigation.md  # researched approaches and scope coverage
  research/         # supporting source notes
  design.md         # technical design, acceptance criteria, and PR plan
  feasibility/      # retained experiments, including disproven assumptions
  verification/     # screenshots, diagnostics, and rerun notes
  decisions.md      # append-only human decisions and implementation deviations
  execution/        # one handoff per PR
  review/           # one round file per reviewed PR revision
  orchestration-summary.md # phase reports when orchestrate is used
```

Read `config.md` and any `config.local.md` override for the vault location.
Use the repository name as the project and a short kebab-case topic slug.
Create files only when they have content. Evidence belongs in the vault,
not the application repository.

Resolve a topic from the worktree's gitignored `.topic` file, then the branch
recorded in topic state. Ask if neither identifies it. New work creates the
topic and marker, using phase `grilling`. Use the current checkout; ask before
creating a worktree. A standalone research topic may start at `investigating`,
but research never supplies a missing approval.

## Brief handoff

For coding work, use `../grill-me/SKILL.md` to settle intent, outcomes, scope,
and constraints. Leave the implementation approach to investigation and design.
Reuse existing answers when resuming. A standalone discussion needs no topic.

Before the interview's final confirmation, save `brief.md` with the problem,
intended users, desired outcomes, required capabilities with stable IDs and
examples, exclusions, constraints, accepted trade-offs, and open research
questions. Include a compact record of the human's answers, distinct from
the agent's recommendations.

Commit and present the brief for confirmation under the approval rules below.
Record that confirmation against its checksum and set phase `investigating`.
This is the interview's confirmation, not an additional approval.

## Approvals and transitions

| Phase | Required input | Human decision | Next phase |
|---|---|---|---|
| grilling | Idea and existing context | Confirm the saved brief | investigating |
| investigating | Confirmed brief | Choose the saved approach and scope | designing |
| designing | Chosen approach | Approve the saved technical design | design-approved |
| executing | All three current approvals | None between PRs; stop for material drift | in-review after the whole stack exists |

Save the draft and present it before requesting its decision. Use
`../wait-what/SKILL.md` for checkpoint explanations. Clear approval in chat
counts without a magic phrase. Silence, a recommendation, a saved draft, or
a worker's decision never counts.

Record each approval in `_state.md` with the date, the human's decision, and
the SHA-256 of the exact approved document. Use `shasum -a 256 <file>`.
Keep approval metadata outside the document to avoid changing its checksum.
Record the selected option in state and `decisions.md`; the investigation
can retain the alternatives the human saw.

Before advancing or resuming, compare approved checksums with current files.
A mismatch pauses dependent work. Preserve prior approvals in Git and the
decision log, then request approval of the revised document. Brief changes
invalidate approach and design approval. Approach changes invalidate design
approval. Design changes invalidate design approval. Supporting research and
execution evidence do not invalidate unchanged documents by themselves.

A human can explicitly waive a checkpoint for a particular run. Record the
waiver and its scope as a waiver, never as approval. "Orchestrate", "continue",
or an old delegated approval does not waive a checkpoint.

### Existing topics

Preserve persisted state and artifacts. A legacy `design_approved` date or
singular `branch` and `pr` remains historical evidence, not a checksum approval.
Resolve its existing PR before adding a `prs` entry. Do not recreate it.
If history identifies the exact human-approved document revision and it still
matches, carry that approval forward with its checksum and recorded provenance.
Never infer brief or approach approval from a design date alone.
For remaining implementation, reuse prior answers and evidence to prepare any
missing brief or checkpoint. Ask only for missing or changed approvals. Never
invent human approval or discard legacy records silently.

## State

Keep these fields in `_state.md` YAML frontmatter, followed by a Markdown
session log. Resolve skill-relative paths against the file that names them.

```yaml
topic: <slug>
project: <project>
phase: grilling # investigating | designing | design-approved | executing | in-review | done
issue: <optional tracker URL>
approvals:
  brief: {date: <date>, sha256: <hash>, decision: <human confirmation>}
  approach: {date: <date>, sha256: <hash>, decision: <chosen option and scope>}
  design: {date: <date>, sha256: <hash>, decision: <human approval>}
prs:
  - id: pr-1
    branch: <branch>
    base: <base branch>
    depends_on: []
    status: planned # implementing | open | blocked
    url: <once open>
    head: <verified commit>
    cleanup_commits: []
    handoff: execution/pr-1.md
execution:
  active_pr: <PR ID while implementing>
  active_agent: <worker ID while running>
```

Omit records until they exist. PR IDs come from the approved design. Record
a deslop no-op in the handoff, rather than making an empty cleanup commit.
Append one log line per session or transition. A PR stack being ready does
not mean it has merged. Set `done` only on the human's confirmation.

For review, identify the PR and its base and head in each
`review/round-<n>.md`. Pick the latest round for that PR, not another stack
member. Review-fix appends verdicts and outcomes to the selected round.
Never replace an earlier review's findings or history.

## Revisions and drift

Preserve the old documents and evidence before revising them. Append the
decision, reason, and affected scope to `decisions.md`. Mark deliberate
deviations explicitly.

- A changed goal returns to `grilling`.
- An invalid approach or failed feasibility returns to `investigating`.
- A changed implementation contract within the chosen approach returns to
  `designing`.

Clear the affected approvals and commit the transition. Retain unaffected
requirements, completed PRs, and disproven experiments. Reapproval concerns
the changed document and invalidated downstream checkpoints.

During execution, minor implementation details can change with a recorded
reason. Stop affected work for changed acceptance criteria, public contracts,
dependencies, security, storage or migration plans, workers or jobs, rollout
requirements, PR boundaries, or a disproven design assumption. Present the
consequence and options; wait for the human. A worker reports drift to its
coordinator and returns without deciding on the human's behalf.

## Ownership and commits

The coordinator owns topic state, approvals, and vault commits. Workers write
assigned artifacts and return handoffs without committing the vault. PR workers
own their application commits. Standalone phase sessions act as coordinator.
When a delegated phase reaches a human decision, it returns the draft and
questions. The coordinator performs that phase's presentation and approval
steps. Delegation changes who presents the decision, never who makes it.

Find the vault Git root with `git -C <vault_path> rev-parse --show-toplevel`.
Commit meaningful drafts, revisions, evidence, decisions, and state before
waiting or handing off. Use `docs(<topic>): <what changed>`. Stage only this
topic's changes, preserving unrelated staged work. Do not amend or squash
earlier document checkpoints. If a commit fails, retain the files and report
the missing Git checkpoint; a checksum can still identify an approved file.
Publishing the vault needs separate authorization.

Revise the same document for the same subject. Git holds previous versions;
do not create numbered copies. Keep reasons for changes in `decisions.md`.

Application red, green, cleanup, and any human-requested simplification commits
remain separate throughout PR preparation and review. Never amend, squash, or
fixup cleanup into implementation. No empty cleanup commits and no AI attribution.

## Orchestration

`../orchestrate/SKILL.md` coordinates the same approvals and drift policy.
It presents reports in chat and saves them in the vault. It cannot approve
for the human. Execution uses one fresh worker per PR and pauses after the
full stack exists. Review starts only after the human continues from that pause.
