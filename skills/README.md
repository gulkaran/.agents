# Skills

My skills for taking a change from investigation through design,
implementation, and review. I approve the approach and the design before the
agent writes code, and I decide when a PR merges.

## Workflow

```
                        ┌───────────────────────┐
    ┌──────────────────▶│     /investigate      │
    │                   └───────────┬───────────┘
    │                               │ approve the approach
    │                               ▼
    │                   ┌───────────────────────┐
    └───────────────────┤        /design        │◀────────┐
      spike fails or    └───────────┬───────────┘         │
      the approach                  │ approve the design  │
      is unsettled                  │                     │ material drift
                                    ▼                     │
                        ┌───────────────────────┐         │
                        │       /execute        ├─────────┘
                        └───────────┬───────────┘
                                    │ draft PRs open
                                    ▼
                        ┌───────────────────────┐  a criterion fails:
                        │        /verify        ├──┐ fix branch and verify again
                        └───────────┬───────────┘◀─┘
                                    │ verification passes
                                    ▼
                        ┌───────────────────────┐
                        │        /deslop        │
                        └───────────┬───────────┘
                                    │
                                    ▼
                        ┌───────────────────────┐
                        │  /pr-review (one PR)  │◀───────────────────────────────┐
                        │ /stack-review (stack) │                                │
                        └───────────┬───────────┘                                │
                                    ├───────────────────────────┐                │
                        no findings │                           │ findings       │ re-review the
                                    ▼                           ▼                │ fixed revision
                        ┌───────────────────────┐   ┌───────────────────────┐    │
                        │         merge         │   │      /review-fix      ├────┘
                        └───────────────────────┘   └───────────────────────┘
```

| Skill                                 | Writes                                                                |
| ------------------------------------- | --------------------------------------------------------------------- |
| [investigate](investigate/SKILL.md)   | `investigation.md`: problem, confirmed requirements, chosen approach. |
| [design](design/SKILL.md)             | `design.md` and `feasibility/`: implementation contract and PR plan.  |
| [execute](execute/SKILL.md)           | One draft PR per planned PR, built by a fresh worker, tests first.    |
| [verify](verify/SKILL.md)             | Results and screenshots for the PR description.                       |
| [deslop](deslop/SKILL.md)             | A separate cleanup commit, or nothing.                                |
| [pr-review](pr-review/SKILL.md)       | `review/round-<n>.md` for my PRs; a chat report for coworkers' PRs.   |
| [stack-review](stack-review/SKILL.md) | One report for a stack, with no finding repeated across PRs.          |
| [review-fix](review-fix/SKILL.md)     | One commit per approved fix, and replies on the review threads.       |

A design spike that disproves the approach stops design and returns to
investigate. Material drift during execute stops the stack for my decision;
there are no approval pauses between PRs. `--stateless` on `pr-review` or
`stack-review` writes no files, for PRs I will not fix myself.

## Supporting skills

- [tdd](tdd/SKILL.md) defines the red and green cycles execute and review-fix use.
- [browser](browser/SKILL.md) drives Playwright CLI or Chrome DevTools CLI for verify.
- [create-verification-skill](create-verification-skill/SKILL.md) and
  [maintain-verification-skill](maintain-verification-skill/SKILL.md) build and
  maintain the app profiles verify loads.
- [opening-a-pr](opening-a-pr/SKILL.md) writes the PR title and description.
- [code-simplification](code-simplification/SKILL.md) runs only when I ask, in
  its own commits.
- [research](research/SKILL.md), [prototype](prototype/SKILL.md), and
  [domain-modeling](domain-modeling/SKILL.md) answer focused questions.
- [grill-me](grill-me/SKILL.md) stress-tests a plan or idea.
- [how](how/SKILL.md), [why](why/SKILL.md), and [teach](teach/SKILL.md) explain
  how code works and why it was built that way.
- [wayfinder](wayfinder/SKILL.md), [to-spec](to-spec/SKILL.md), and
  [to-tickets](to-tickets/SKILL.md) plan work too large for one session.
- [technical-writing](technical-writing/SKILL.md),
  [unslop](unslop/SKILL.md), and [simple-english](simple-english/SKILL.md) set
  the writing style.
- [thermo-nuclear-code-quality-review](thermo-nuclear-code-quality-review/SKILL.md)
  is a strict maintainability review.

## Topics and accio

Each piece of work is a topic folder in the vault:

```
<vault>/projects/<project>/topics/<slug>/
  investigation.md  design.md  decisions.md
  research/  feasibility/  verification/  review/round-<n>.md
```

[accio](../accio/README.md) owns these paths. Skills never build them by hand:

| Command                     | Use                                                        |
| --------------------------- | ---------------------------------------------------------- |
| `accio topic use <slug>`    | Start or switch the topic for this checkout.               |
| `accio topic current`       | Print the current topic.                                   |
| `accio path <artifact>`     | Print where an artifact goes, with `--next` for a review.  |
| `accio commit -m "<msg>"`   | Commit the topic folder in the vault. It never pushes.     |
| `accio config get tracker`  | Print the tracker; recipes are in `_shared/trackers/`.     |

Application history keeps red tests, green implementation, deslop, and any
requested simplification in separate commits.

The [verification protocol](_shared/verification.md) defines where verify
finds app profiles and stores evidence. Evidence images go into PR
descriptions through GitHub CLI and are never committed to the repo.

[Skill style](_shared/skill-style.md) is the guide for writing a `SKILL.md`.

## Setup

1. Clone this repository to `~/.agents`.
2. Link `~/.claude/skills` to `~/.agents/skills`.
3. Install accio and point it at the vault, as the
   [accio README](../accio/README.md) describes:

   ```sh
   accio vault set ~/Desktop/vault
   accio config set tracker github
   ```

The vault is its own Git repository. Skills commit to it locally; I push it
myself.

Start new work with `/investigate <what to build or fix>`.
