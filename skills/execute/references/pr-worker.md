# PR worker contract

The execute coordinator launches one fresh worker per planned PR with
`fork_turns: none`, or the host's equivalent. The worker builds only that PR.

## Input

Pass the repository path, the topic path from `accio topic current --path`,
and the approved investigation, design, and decisions.

Pass the PR ID, scope, acceptance criteria, test boundaries, branch and base,
dependencies, and verified handoffs from completed PRs. Include feasibility
evidence, known gaps, and any model override. Pass these instructions as the
worker's task; do not ask it to invoke execute recursively.

Include the absolute evidence directory from `../../_shared/verification.md`
and any known app profile path. Profile discovery happens during verification
and does not require a work/personal answer.

## Work

1. Confirm the assigned checkout is safe to use. Read repository instructions.
   Create or select the assigned branch from the verified base. Never discard
   unrelated changes to switch branches.
2. Before implementation, capture relevant before evidence when tools allow,
   using `../../verify/SKILL.md` in baseline-only mode to discover an app skill
   or use the generic flow.
   Keep it in the supplied evidence directory and record the baseline revision.
   Read `../../tdd/SKILL.md`. Implement one behavior at a time, preserving red
   and green commits. Derive expected results from the approved examples.
   Observe the test fail for the missing behavior and commit red before coding.
   Commit green after it passes; do not push a failing intermediate head.
   Never weaken assertions to make the implementation pass.
   Record post-red test edits and their reasons.
3. Use the design's feasibility evidence. A new assumption that could invalidate
   the design needs a focused experiment and a drift report. Run
   `accio path feasibility --name <assumption>.md` and keep the experiment
   notes at the printed path.
4. Save screenshots and useful diagnostics in the supplied evidence directory.
   Missing optional capture is a reported gap, not a fabricated substitute.
5. Keep production changes within the PR scope. Keep required or explicitly
   requested product docs and assets; scratch probes and local verification
   artifacts stay out of application commits.
6. Run the required checks. Commit the working implementation before cleanup.
   Run `../../deslop/SKILL.md` against this PR's base, verify the cleanup, and
   keep its commit separate. Do not invoke code simplification.
7. Launch a fresh verification agent with no inherited conversation. Give it
   `../../verify/SKILL.md`, repository and evidence paths, any known app profile,
   acceptance criteria, required
   checks and their existing results, base and final head, and before captures.
   Give it no authority to edit product code or tests, commit, or open PRs.
   It discovers and reads the app skill when present and uses generic discovery
   otherwise.
   Wait for its results while keeping the checkout and app build unchanged.
   Missing delegation or a required failed/blocked check blocks completion.
   Repair failures in implementation, commit separately, repeat deslop as
   needed, and rerun affected verification on the new head before proceeding.
8. Open the PR with `../../opening-a-pr/SKILL.md`, using the assigned base.
   Pass the verification summary, gaps, inspected screenshot paths, and captions
   so the PR description contains uploaded evidence. Verify the posted body.
   Verify that the diff contains only this PR's changes. Do not merge it.
9. Return the handoff below. Do not launch the next worker, approve a change,
   or commit the vault.

Material drift or a required check that cannot pass stops this worker. Return
the partial handoff and the blocker to the coordinator. Do not drop
acceptance criteria or conceal failures to open a PR.

## Handoff

Return:

- PR ID, scope, branch, base, URL, and verified head.
- Requirement IDs delivered and any remaining work.
- Red and green commit pairs, and justified edits to earlier tests.
- Required checks, their tested revision, results, and verification gaps.
- Verification agent outcome, resolved evidence directory, screenshot upload
  results, and any evidence unavailable in the PR description.
- Deslop commit links or a verified no-op.
- Deviations and dependency details the next worker needs.

The coordinator owns vault commits. The worker may append to `decisions.md`
but cannot turn its own recommendation into human approval.

## Resume

Reconcile an interrupted worker before launching another writer. Reuse a live
worker or inspect its branch after interruption. A replacement gets a fresh
context with the partial handoff the interrupted worker returned. Check for an
existing PR before opening one. Preserve completed commits and cleanup
separation.
