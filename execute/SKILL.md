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

## Per slice, in the design's stack order

### 1. Tests first (red)

- Capture the current behavior before implementation when practical, so the
  PR can show a comparable before/after result.
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

1. Collect concrete evidence for each meaningful behavior change for the
   PR's **What changed** section, following `~/.agents/skills/_shared/pr.md`.
   Compare the baseline captured before implementation with the after
   behavior under comparable conditions. Use screenshots
   for visible changes when capture tools are available, or test output,
   request/response examples, or measured data as appropriate. Test commands
   alone are instructions, not evidence of a successful run. Preserve useful
   raw evidence in the topic's vault folder and include reviewer-accessible
   evidence in the PR itself. If capture or upload tools are unavailable,
   use the strongest available evidence and state the verification limits;
   never invent proof or treat missing screenshot tooling alone as an
   approval gate.
2. Read `~/.agents/skills/_shared/style.md` immediately before reporting.
   Explain the features delivered and verification results in simple terms.
   Call out deviations and post-red test changes with their importance and
   justification in the agent session, not in the PR description. Keep their
   durable record in `decisions.md`. Save detailed test output and diffstats
   in an execution appendix to `design.md`.
   Reread the style contract before a revised report after further work.
3. Push and open PRs automatically per the PR style doc and the approved
   design's PR grouping. Do not wait for a separate pre-PR approval or write
   a `pre_pr_approved` stamp. The design approval and medium+ drift rules
   above still apply.
   Multiple implementation slices may share a PR; do not create a PR per
   session or test step. Record the PR URL(s) in `_state.md`, phase →
   `in-review`, log line. Tell the human the topic is ready for `self-review`.
