# PR worker contract

The execute coordinator launches one fresh worker per planned PR with
`fork_turns: none`, or the host's equivalent. The worker builds only that PR.

## Input

Pass the repository and topic paths, the shared topic protocol, and the
approved brief, investigation, design, and decisions. Include their approved
checksums and the selected approach.

Pass the PR ID, scope, acceptance criteria, test boundaries, branch and base,
dependencies, and verified handoffs from completed PRs. Include feasibility
evidence, known gaps, and any model override. Pass these instructions as the
worker's task; do not ask it to invoke execute recursively.

Include the human's work/personal answer for this run, selected app profile or
its absence, and absolute evidence directory from `../../_shared/verification.md`.
Without that answer, return to the coordinator before any tool call. Never
infer the context or reuse an answer from an earlier run.

## Work

1. Confirm the assigned checkout is safe to use. Read repository instructions
   and verify approval checksums. Create or select the assigned branch from
   the verified base. Never discard unrelated changes to switch branches.
2. Before implementation, capture relevant before evidence when tools allow,
   using the app profile and `../../browser/SKILL.md` for browser applications.
   Keep it in the supplied evidence directory and record the baseline revision.
   Read `../../tdd/SKILL.md`. Implement one behavior at a time, preserving red
   and green commits. Derive expected results from the approved examples.
   Observe the test fail for the missing behavior and commit red before coding.
   Commit green after it passes; do not push a failing intermediate head.
   Never weaken assertions to make the implementation pass.
   Record post-red test edits and their reasons.
3. Use the design's feasibility evidence. A new assumption that could invalidate
   the design needs a focused experiment and a drift report. Keep experiment
   files in the topic's `feasibility/` folder.
4. Save screenshots and useful diagnostics in the supplied evidence directory.
   Missing optional capture is a reported gap, not a fabricated substitute.
5. Keep production changes within the PR scope. Keep required or explicitly
   requested product docs and assets; scratch probes and local verification
   artifacts stay out of application commits.
6. Run the required checks. Commit the working implementation before cleanup.
   Run `../../deslop/SKILL.md` against this PR's base, verify the cleanup, and
   keep its commit separate. Do not invoke code simplification.
7. Launch a fresh verification agent with no inherited conversation. Give it
   `../../verify/SKILL.md`, the confirmed work/personal answer, repository and
   evidence paths, app profile or its absence, acceptance criteria, required
   checks and their existing results, base and final head, and before captures.
   Give it no authority to edit product code or tests, commit, or open PRs.
   It reads the app skill when present and uses generic discovery otherwise.
   Wait for its results while keeping the checkout and app build unchanged.
   Missing delegation or a required failed/blocked check blocks completion.
   Repair failures in implementation, commit separately, repeat deslop as
   needed, and rerun affected verification on the new head before proceeding.
8. Open the PR with `../../opening-a-pr/SKILL.md`, using the assigned base.
   Pass the verification summary, gaps, inspected screenshot paths, and captions
   so the PR description contains uploaded evidence. Verify the posted body.
   Verify that the diff contains only this PR's changes. Do not merge it.
9. Write `execution/<pr-id>.md` and return. Do not launch the next worker,
   approve a change, or commit the vault.

Material drift or a required check that cannot pass stops this worker. Save
the partial handoff and report the blocker to the coordinator. Do not drop
acceptance criteria or conceal failures to open a PR.

## Handoff

Record:

- PR ID, scope, branch, base, URL, and verified head.
- Requirement IDs delivered and any remaining work.
- Red and green commit pairs, and justified edits to earlier tests.
- Required checks, their tested revision, results, and verification gaps.
- Verification agent outcome, resolved evidence directory, screenshot upload
  results, and any evidence unavailable in the PR description.
- Deslop commit links or a verified no-op.
- Deviations and dependency details the next worker needs.

The coordinator owns state and vault commits. The worker may append decisions
but cannot turn its own recommendation into human approval.

## Resume

Reconcile a recorded worker before launching another writer. Reuse a live
worker or inspect its branch and handoff after interruption. A replacement
gets a fresh context with the saved partial handoff. Check for an existing PR
before opening one. Preserve completed commits and cleanup separation.
