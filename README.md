# Skills

My skills for taking software ideas through research, design, implementation,
and review. The agent does the legwork; I choose the approach, approve the
design, and decide which review fixes to make.

## How they work

The main workflow is `investigate → design → execute → self-review → review-fix`.

| Skill | What it does |
|---|---|
| [investigate](investigate/SKILL.md) | Explores the problem and codebase, then explains the options so I can choose an approach. |
| [design](design/SKILL.md) | Checks risky assumptions and defines the behavior, acceptance criteria, and PR plan for approval. |
| [execute](execute/SKILL.md) | Implements the approved design with tests first, collects evidence, and opens PRs after verification. |
| [self-review](self-review/SKILL.md) | Reviews the diff, whole files, and connected code for merge blockers and useful quality improvements. |
| [review-fix](review-fix/SKILL.md) | Verifies findings, proposes fixes for approval, and implements them with a clear record. |

For larger or less defined work, `wayfinder` maps decisions, `to-spec` turns
the discussion into a spec, and `to-tickets` creates actionable issues.
`grilling`, `research`, `domain-modeling`, and `prototype` help answer specific
questions along the way. `teach` explains a body of work using `how` for
architecture and runtime flow and `why` for evidence behind design decisions.
See [VENDORED.md](VENDORED.md) for upstream credits.

## Why this works for me

- **Context survives between sessions.** Research, designs, decisions, and
  reviews live together in a topic folder in my knowledge vault.
- **Document history survives revisions.** Meaningful updates are committed
  together in the vault. Investigation lives in `investigation.md`, supporting
  notes in `research/`, and execution evidence in `verification/`. Failed
  feasibility returns to investigation and invalidates the old design approval.
- **Tests follow the intended behavior.** Failing tests are committed before
  implementation, and later test changes need a recorded explanation.
- **Review respects the design.** Deliberate decisions stay visible, while
  blockers, duplication, unnecessary code, and smaller improvements get reported.
- **Reports support decisions.** Plain-language summaries explain what matters;
  saved artifacts preserve the detailed evidence.
- **Approval happens at meaningful points.** Implementation follows an approved
  design, and significant changes to that design come back for a decision.

## Setup

Run `./install.sh` in Bash to symlink the skills into `~/.agents/skills`.
Set the vault path and tracker in [_shared/config.md](_shared/config.md).
For machine-specific settings, use `_shared/config.local.md` (gitignored).
The vault must have its own initialized Git repository. Workflow skills commit
local document checkpoints using [_shared/topic.md](_shared/topic.md); vault
publishing remains separate from application pushes and PRs.

Invoke a skill by name, for example: `Use $investigate to explore this idea.`
