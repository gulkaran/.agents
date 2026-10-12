---
name: self-review
description: Analysis-only review of a topic's changes for merge blockers and actionable code-quality improvements across the diff, whole files, and connected codebase. Uses recorded design decisions and checks test integrity against red commits. Writes review.md; changes no code. Use after execute has opened the PR (or pre-PR on request).
disable-model-invocation: true
---

# Self-review

Find what genuinely blocks a merge and what would meaningfully improve the
change, including small cleanups. Use the topic's context so conscious choices
don't get flagged as mistakes. Analysis only: write the review and state log;
never edit code or tests.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.

## Preconditions

Resolve the topic. Load `design.md` and `decisions.md` **before reading any
code** — they are the lens for every judgment below.

Establish the review range from the topic's PR or branch and include relevant
uncommitted work when requested. Record the base and reviewed revision, plus
any working-tree changes included, so findings have an unambiguous scope.

## Process

### 1. Three passes

- **Diff pass:** inspect the changes against `design.md` and its acceptance
  criteria. Identify missing promises, unexplained additions, and local defects.
- **Whole-file pass:** read changed functions and their surrounding files,
  including tests. Check whether the change fits the module's responsibilities
  and whether existing code or tests became unnecessary.
- **Connected-codebase pass:** follow affected callers, dependencies, and shared
  contracts. Search for existing equivalents before proposing new helpers.
  Trace changed behavior through its consumers; verify design commitments that
  span files. Expand only as needed to assess this topic, not into an unrelated
  repository cleanup.

Apply these lenses across all three passes; do not repeat a finding per pass:

- **Behavior and safety:** incorrect results, boundary inputs, broken consumers,
  unsafe input handling, and lost or partially written data.
- **Operational behavior:** concurrency, resource growth, and costly work at the
  documented workload. State any workload assumption behind a finding.
- **Test value:** meaningful protection for risky behavior; tests that merely
  mirror implementation or provide no distinct regression protection.
- **Code economy:** unused paths, unnecessary configuration or dependencies,
  existing helpers or platform facilities that replace custom logic, duplicate
  methods that can drift, and abstractions without a current purpose. Separate
  unrelated responsibilities where that clarifies behavior; avoid extra helpers
  created only to shorten functions.

**Test integrity remains mandatory.** Diff every test file touched by the topic
(including deleted files) against its slice's red commit. A post-red test change
without a matching justification in `decisions.md` is an automatic finding;
classify it by its actual consequence. Judge whether tests still enforce the
*wanted* behavior rather than the implementation that happens to exist. If red
history is unavailable, record that limitation instead of claiming this check
passed.

For duplicate tests, compare the behavior, input partition, assertion, and test
boundary they protect. Similar setup or assertions alone do not prove redundancy.
Name the retained test and explain why consolidation loses no distinct coverage.
A cleanup recommendation does not justify an earlier unexplained test change;
keep the integrity finding until the decision is recorded.

### 2. Context check — before anything becomes a finding

Cross-check every candidate finding against `decisions.md`. If it's a recorded
deliberate deviation, it is **not a finding**; record it once under "Verified
deliberate" in `review.md` so the check is preserved. Only unexplained departures
survive as findings.

Verify that the candidate is actually covered by the decision: an accepted
trade-off does not excuse a separate implementation defect. Record any limits
of the decision that matter to the review.

For other candidates, confirm the relevant code paths and evidence. Defects
need a reachable failure scenario; cleanup findings need a concrete maintenance
cost or removable redundancy, not an invented runtime failure. Propose the
smallest useful fix. Exclude taste-only preferences and speculative redesigns.
Report issues introduced, worsened, or directly exposed by the topic; distinguish
pre-existing causes when relevant.

Classify findings independently of how easy they are to fix:

- **Must fix:** a merge blocker, with the violated requirement or concrete
  unacceptable consequence stated.
- **Should fix:** a substantive improvement worth doing even if merging is safe.
- **Nice to have:** a small, evidenced improvement, including trivial cleanup.

Do not discard an actionable finding because it is non-blocking, or promote a
cleanup to a blocker merely to get it reported. Group repeated instances of the
same cause, retaining all relevant locations.

### 3. Report

Read `~/.agents/skills/_shared/style.md` now, immediately before reporting.
Lead with whether anything blocks merging. Explain blockers through their
practical consequences and recommended fixes; use a concrete failure example
where useful. Include non-blocking improvements in chat as compact, grouped
lines so they remain visible without burying blockers. Mention deliberate
deviations when they affect the human's decision.

Keep the complete findings and "Verified deliberate" checks in `review.md`,
with evidence and locations in its appendix. Group findings by the three
importance levels above, omitting empty groups. Give each finding a stable ID,
the affected behavior or code, its consequence or cleanup benefit, and the
recommended fix. Put paths, line references, decision references, and supporting
checks in the appendix, keyed to those IDs. Preserve existing IDs and resolved
outcomes when revising a review.

Record the review scope, verification performed, and meaningful gaps. Distinguish
"no blockers found" from "fully verified" when checks were unavailable. Log one
line in `_state.md`. Recommend `review-fix` for actionable findings, making clear
which are required before merge and which are optional; changes remain untouched.
