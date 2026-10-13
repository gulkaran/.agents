# Topic protocol

A **topic** is one unit of work moving through the pipeline
(investigate → design → execute → self-review → review-fix). All of its
artifacts live in one folder in the knowledge vault, and one state file tracks
where it stands. Every workflow skill reads this protocol, resolves the topic
first, and updates state last. Skills never require the human to pass file
paths around; state on disk is the shared memory. Commit meaningful artifact
changes under the versioning rules below, including drafts before a handoff.

## Topic folder

```
<vault_path>/<project>/topics/<slug>/
  _state.md         # phase, approvals, links: read first, write last
  investigation.md  # synthesis and approach choice, written by investigate
  research/         # supporting notes: <subject>.md, written by research
  design.md         # draft or approved design, including feasibility appendix
  feasibility/      # retained design experiments, including disproven ones
  verification/     # execution/review results, screenshots, and rerun details
  decisions.md      # append-only decision log
  review.md         # written by self-review, updated by review-fix
  orchestration-summary.md # appended phase reports during orchestration
```

- `<project>` is the repo name unless the human says otherwise.
- `<slug>` is short and kebab-case, derived from the idea/issue title.
- Create files and subfolders only when there is content to save.

## Orchestrated runs

When the user requests `orchestrate`, follow
[`orchestrate/SKILL.md`](../orchestrate/SKILL.md) for approvals, delegation,
reports, and stopping conditions. Pass that file to every phase agent.

- Its approval rules replace the interactive gates in this protocol and the
  phase skills for that run. Record decisions as delegated choices, not human replies.
- Each phase still reads `style.md`. Append its normal report to
  `orchestration-summary.md` instead of returning the report in chat.
- The orchestrator commits vault changes after each phase. Phase agents make
  the application commits required by `execute` and `review-fix`.
- Keep the topic `in-review` when automation finishes. Record the orchestration
  outcome separately. A reviewed PR does not mean it has merged.
- Standalone skill calls keep their existing approval and reporting rules.

## Document versions and commits

These rules apply to documents and retained artifacts written in the configured
vault. Find its Git root with `git -C <vault_path> rev-parse --show-toplevel`;
run vault Git commands there, separately from the application repository. If
the vault is not a Git repository or a commit fails, preserve the files and
report the missing checkpoint instead of claiming the revision was saved in Git.

- Commit every meaningful creation or revision before handing back, waiting
  for a decision, or replacing an approach. Group related documents, retained
  artifacts, decisions, and the state update in one commit. Do not commit each
  intermediate edit or create empty commits. Mark unapproved work as draft;
  saving or committing it never grants approval.
- Update the same document for the same subject. Git holds earlier versions;
  do not create numbered copies. Keep the reasons for changed approaches in
  `decisions.md`, with links to the evidence, even when the old text is replaced.
  Checkpoint any unsaved earlier approach before replacing it.
- Review the diff and stage only this task's files or hunks. Do not sweep in
  another topic's changes or pre-existing edits. Leave unrelated staged work
  intact and exclude it from the commit. The coordinating agent commits after
  its workers finish; workers write assigned artifacts without committing.
- Use `docs(<topic>): <what changed>` and explain why in the commit body when
  the diff alone is insufficient. When the current session ID is reliably
  available, include `Session: <id>` in the body. Omit it if unavailable; never
  guess it. Store essential reasoning in the documents because a session link
  may later be unavailable. Do not add co-author or AI attribution trailers.
- Keep checkpoints as separate commits; do not amend or squash earlier
  document revisions. Local vault commits are authorized by this workflow;
  pushing or publishing the vault requires separate user authorization.

To compare approaches, inspect `git log --follow -- <document>` and
`git diff <older-commit> <newer-commit> -- <document>`, using paths relative to
the vault Git root. Read both versions with `git show <commit>:<path>` plus
the corresponding `decisions.md` and evidence. Account for renames when the
older path differs. This comparison must work without the original chat.

## Reopening investigation

Failed feasibility or a human-requested approach change can return an existing
topic to investigation. This is an allowed backward transition, not a new topic.

1. Preserve the failed experiment and record what invalidated the approach in
   `decisions.md`. Checkpoint the old documents if they have uncommitted changes.
2. Mark any existing `design.md` as superseded/pending a new design and clear
   `design_approved` in `_state.md`. Set phase `investigating`, append the reason
   to the log, and commit the transition. Historical approval remains in Git
   and the log; it cannot authorize the replacement design.
3. Continue focused research in `research/<subject>.md` and revise
   `investigation.md` in place, preserving prior constraints and evidence that
   still apply. Commit meaningful revisions while the approach is pending.
4. Record the human's new approach choice before moving to `designing`.
   A revised design requires fresh explicit approval before execution resumes.

Supporting research alone preserves the current phase and approvals. If it
invalidates the approved approach, raise the finding and use this transition
through design or the human's direction; never silently continue execution.

When the human requests a contract revision without changing the chosen
approach, return to `designing`, clear `design_approved`, mark the design as
draft, and commit. Investigation need not repeat when its findings still hold.

## Resolving the current topic

In order:

1. A `.topic` file at the worktree root (gitignored) containing the absolute
   path of the topic folder.
2. A branch named `<category>/<slug>` (or a branch recorded in some topic's
   `_state.md`).
3. Ask the human.

When a skill **creates** a topic it writes the `.topic` marker so every later
session in that worktree self-locates. Name new branches `<category>/<slug>`.
Use the current checkout; ask the human before creating a worktree.

## `_state.md`

```markdown
---
topic: <slug>
project: <project>
phase: investigating | designing | design-approved | executing | in-review | done
design_approved: <date, once given in chat>
issue: <tracker URL, if any>
branch: <git branch>
pr: <PR URL, once open>
---

## Log

- <date> <skill>: <one line — what happened / phase change>
```

Rules:

- Read `_state.md` **first**. If the phase doesn't match the skill being
  invoked (e.g. `execute` without `design-approved`), say so and stop — the
  human can explicitly override. The reopening and design revision transitions
  described above are explicit exceptions.
- Approvals are given **in chat** ("approved") and recorded here with a date.
  A recorded approval is what downstream skills trust.
- Append one log line per session or phase change. The log is for humans.

## `decisions.md`

Append-only. One entry per decision, including mid-execution drift:

```markdown
## <date> — <short decision title>

- **Decision:** what was chosen.
- **Why:** the reasoning, in one or two lines.
- **Deliberate deviation:** yes/no — set yes when this knowingly departs from
  industry-standard practice or from the approved design.
```

This file is the context that keeps later phases honest: self-review reads it
and does not flag deliberate deviations as findings.

## Drift policy (during execute)

- **Minor drift** — implementation detail differs from design but no
  acceptance criterion, public interface, or dependency changes: record it in
  `decisions.md` and continue.
- **Medium+ drift** — an acceptance criterion, public interface, dependency,
  or anything security-relevant changes, or a design assumption turns out
  false: **stop**, raise it in chat with the options, and wait for the human.
