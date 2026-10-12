---
name: execute
description: Implement an approved design in TDD slices - behavior tests committed red first, then implementation to green. Collect behavior evidence and automatically open PRs when verification passes. Use only on a topic with a recorded design approval.
disable-model-invocation: true
---

# Execute

Implement the approved design, slice by slice, tests first. Git history is the
evidence that tests came before implementation. Once verification passes,
push and open PRs without a separate PR-creation approval.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`,
`~/.agents/skills/_shared/pr.md`.

## Preconditions

Resolve the topic. **Refuse to run without `design_approved` in `_state.md`**
— point the human at `design` instead. Work on the branch recorded in
`_state.md` (create it if needed). Set phase `executing`.

Keep useful verification artifacts, such as screenshots or diagnostics worth
retaining, under the topic's `verification/` folder. Create it only when there
is something useful to save; routine passing test logs and Git diffstats do
not need archiving. Keep enough context to interpret retained artifacts. Commit
meaningful vault updates at slice boundaries and before handoffs or drift
stops using the topic protocol, separately from application red/green commits.

## Per slice, in the design's stack order

### 1. Tests first (red)

- For visible changes, capture a before screenshot when practical so the PR
  can show a comparable before/after view.
- Derive behavior tests from the slice's **acceptance criteria in
  `design.md`** — the behavior we want, not the implementation that will
  exist. Core coverage of the concepts that matter; not exhaustive
  micro-testing.
- The test author must not see implementation-in-progress. Either write tests
  before any implementation exists, or delegate test authoring to a subagent
  given only `design.md`.
- **Commit the failing tests**: `test(<slice>): red — <behavior>`. This commit
  is the evidence trail; never rewrite it away before the PR exists.

### 2. Implement (green)

- Implement until the red tests pass. Mechanical slices may be delegated to
  faster models; the design contract travels with the delegation.
- **Never weaken a test to make it pass.** Any test edit after the red commit
  requires a one-line justification appended to `decisions.md`. Silent test
  mutations become automatic self-review findings.

### 3. Drift

Apply the topic protocol's drift policy: minor → record in `decisions.md` and
continue; medium+ (acceptance criteria, public interfaces, dependencies,
security) → **stop, raise in chat with options, wait**.

## Verify, report, and open PRs

When the stack (or the agreed set of slices) is green:

1. Run the checks appropriate to the changes and summarize their outcomes.
   For visible changes, capture and inspect screenshots when tools are
   available. Include useful captures in the PR's optional **Screenshots**
   section, following `~/.agents/skills/_shared/pr.md`. Prefer comparable
   before/after views; an after-only screenshot is fine for a new screen or
   when no before capture exists. Missing capture or upload tooling is not
   an approval gate.
2. Read `~/.agents/skills/_shared/style.md` immediately before reporting.
   Explain the features delivered and verification results in simple terms.
   Call out deviations and post-red test changes with their importance and
   justification in the agent session, not in the PR description. Keep their
   durable record in `decisions.md`. A brief summary of checks run, outcomes,
   and meaningful verification gaps is sufficient; no execution appendix in
   `design.md` is required.
   Reread the style contract before a revised report after further work.
3. Push and open PRs automatically per the PR style doc and the approved
   design's PR grouping. Do not wait for a separate pre-PR approval or write
   a `pre_pr_approved` stamp. The design approval and medium+ drift rules
   above still apply.
   Multiple implementation slices may share a PR; do not create a PR per
   session or test step. Record the PR URL(s) in `_state.md`, phase →
   `in-review`, log line. Commit the related vault artifacts and state before
   telling the human the topic is ready for `self-review`.
