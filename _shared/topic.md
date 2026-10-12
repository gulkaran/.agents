# Topic protocol

A **topic** is one unit of work moving through the pipeline
(investigate → design → execute → self-review → review-fix). All of its
artifacts live in one folder in the knowledge vault, and one state file tracks
where it stands. Every workflow skill reads this protocol, resolves the topic
first, and updates state last. Skills never require the human to pass file
paths around — state on disk is the shared memory.

## Topic folder

```
<vault_path>/<project>/topics/<slug>/
  _state.md      # phase, approvals, links — read first, write last
  research.md    # written by investigate
  design.md      # written by design (includes feasibility evidence)
  feasibility/   # retained spike files and evidence, when needed by design
  decisions.md   # append-only decision log
  review.md      # written by self-review, updated by review-fix
```

- `<project>` is the repo name unless the human says otherwise.
- `<slug>` is short and kebab-case, derived from the idea/issue title.

## Resolving the current topic

In order:

1. A `.topic` file at the worktree root (gitignored) containing the absolute
   path of the topic folder.
2. A branch named `topic/<slug>` (or a branch recorded in some topic's
   `_state.md`).
3. Ask the human.

When a skill **creates** a topic it writes the `.topic` marker so every later
session in that worktree self-locates.

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
  human can explicitly override.
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
