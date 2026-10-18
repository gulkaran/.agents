# Skills

My skills for taking an idea through a confirmed brief, research, technical
design, implementation, and review. I confirm intent, choose the approach,
and approve the design before the agent implements it.

## Workflow

| Skill                               | Output and checkpoint                                                                   |
| ----------------------------------- | --------------------------------------------------------------------------------------- |
| [grill-me](grill-me/SKILL.md)       | Saves the brief and asks me to confirm shared understanding.                            |
| [investigate](investigate/SKILL.md) | Researches the brief and presents the complete approach and scope for my choice.        |
| [design](design/SKILL.md)           | Explains technical consequences, tests risky assumptions, and asks for design approval. |
| [execute](execute/SKILL.md)         | Uses a fresh worker per PR, creates the complete stack, then pauses.                    |
| [pr-review](pr-review/SKILL.md)     | Reviews PRs after I continue from the stack handoff.                                    |
| [review-fix](review-fix/SKILL.md)   | Verifies findings and proposes fixes for approval.                                      |

[Orchestrate](orchestrate/SKILL.md) coordinates the same workflow and preserves
its human checkpoints. Material drift pauses affected implementation for a
decision. There are no routine approval pauses between PRs.

## Supporting skills

- [wait-what](wait-what/SKILL.md) explains checkpoint proposals in plain language.
- [tdd](tdd/SKILL.md) defines small behavior-first test and implementation cycles.
- [verify](verify/SKILL.md) checks acceptance criteria in the running app with a
  fresh verification worker during execution.
- [browser](browser/SKILL.md) guides Playwright CLI and Chrome DevTools CLI use.
- [create-verification-skill](create-verification-skill/SKILL.md) and
  [maintain-verification-skill](maintain-verification-skill/SKILL.md) create and
  maintain app profiles, adapted from P-Stack with context and storage changes.
- [deslop](deslop/SKILL.md) runs on every PR and commits cleanup separately.
- [code-simplification](code-simplification/SKILL.md) runs only when I request it.
  Its behavior-preserving refactors have their own commits.
- [opening-a-pr](opening-a-pr/SKILL.md) describes and opens the verified PR.
- [research](research/SKILL.md), [domain-modeling](domain-modeling/SKILL.md),
  and [prototype](prototype/SKILL.md) answer focused questions.
- [teach](teach/SKILL.md) explores how a subsystem works and why it was built
  that way. [wayfinder](wayfinder/SKILL.md), [to-spec](to-spec/SKILL.md), and
  [to-tickets](to-tickets/SKILL.md) support broader planning.

## Shared records

The [topic protocol](_shared/topic.md) stores brief, investigation, design,
decisions, feasibility evidence, execution handoffs, and reviews in the vault.
Approvals identify exact document revisions. A changed requirement invalidates
the affected approvals; completed PRs and earlier evidence remain available.

Application history keeps red tests, green implementation, deslop, and any
requested simplification separate throughout PR preparation and review.

Execution uses `verify` at the verification stage to discover an app-specific
skill or use generic discovery. Only creating or maintaining a verification
skill asks **Is this work or personal?** to choose persistent storage. The
[verification protocol](_shared/verification.md) defines discovery and evidence
locations. Inspected evidence images are uploaded into PR descriptions with
GitHub CLI, never committed just to provide an image URL.

## Setup

Run `./install.sh` in Bash from the source repository to link skills into
`~/.agents/skills`. Set vault and tracker settings in
[\_shared/config.md](_shared/config.md). Put machine overrides in the gitignored
`_shared/config.local.md`.

The vault uses its own Git repository. Workflow skills commit local document
checkpoints; publishing the vault needs separate authorization.

Start with `Use $grill-me to work through this idea`, or use `$orchestrate`
to coordinate the workflow. An existing confirmed topic can resume at its
next phase.
