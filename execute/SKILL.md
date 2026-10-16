---
name: execute
description: Implement an approved design in TDD slices. Behavior tests are committed red first, then code to green; evidence goes to the vault, the branch is desloped, and PRs open automatically when verification passes. Use only on a topic with a recorded design approval.
disable-model-invocation: true
---

# Execute

Implement the approved design slice by slice, tests first. Git history is the proof that tests came before code. The application repo receives only what the design asked for: evidence, notes, and anything you made to convince yourself go to the vault or get deleted before the PR opens.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.

## 1. Resolve the topic

Follow the topic protocol. Refuse to run without `design_approved` in `_state.md` and point the human at `design`. Work on the branch in `_state.md`, creating it if needed. Set phase `executing`.

## 2. Write the tests red

Per slice, in the design's stack order:

- Derive tests from the slice's acceptance criteria in `design.md`: the behavior wanted, not the implementation about to exist. Core coverage of the concepts that matter, not exhaustive micro-tests.
- The test author must not see implementation in progress. Write tests before any code exists, or delegate test authoring to a subagent given only `design.md`.
- Commit the failing tests: `test(<slice>): red — <behavior>`. Never rewrite this commit away before the PR exists.
- Visible change: capture a before screenshot into the topic's `verification/` folder when tools allow.

## 3. Implement to green

- Implement until the red tests pass. Mechanical slices may go to faster models with the design contract attached.
- Never weaken a test to make it pass. Any test edit after the red commit gets a one-line justification in `decisions.md`; a silent edit is an automatic pr-review finding.
- Touch only what the slice needs. No helper, abstraction, flag, or configuration the acceptance criteria do not call for.
- Docs in the repo only when an explicit repo rule requires one for the code path you touched, such as an `AGENTS.md` intent-doc table. Never add guides, references, READMEs, previews, or local-testing notes. A well-written qualification guide still had to be deleted in a follow-up PR.
- Scripts, fixtures, and probes you write to try things out are scratch. Keep them out of commits or delete them in the same slice.

## 4. Handle drift

Apply the topic protocol's drift policy. Minor: record in `decisions.md` and continue. Medium or larger (acceptance criteria, public interfaces, dependencies, security, a false design assumption): stop, raise it in chat with options, and wait.

## 5. Verify

When the stack or the agreed slices are green:

- Run the checks the changes call for and note outcomes and gaps.
- Visible changes: capture and inspect after screenshots when tools allow. Missing capture tooling is not a gate.
- A behavior you cannot verify is one line in the gap list for the human. Never stand in for the real check with a rendered approximation, a guide for someone else to run, or a probe test.
- Retained evidence (screenshots, diagnostics worth keeping, rerun notes) goes under the topic's `verification/` folder in the vault, never in the application repo. Routine passing logs are not retained.
- Commit vault updates at slice boundaries and before handoffs, separately from application commits.

## 6. Deslop, then open PRs

1. Run `~/.agents/skills/deslop/SKILL.md` on the branch.
2. Open PRs with `~/.agents/skills/opening-a-pr/SKILL.md`, following the design's PR grouping. No separate pre-PR approval: design approval and the drift rules are the gates.
3. Record PR URLs in `_state.md`, set phase `in-review`, log, and commit the vault.

## 7. Report

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before reporting, and again before any revised report. Explain what was delivered and how it was verified. State deviations and post-red test edits with their justification here and in `decisions.md`, not in the PR. Tell the human the topic is ready for `pr-review`.
