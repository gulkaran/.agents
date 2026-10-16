---
name: review-fix
description: Verify review findings from review.md or PR review threads, propose the smallest safe fixes for approval, implement each as its own commit, then reply to and resolve every thread on the PR. Use after pr-review or when a PR has review comments to address.
disable-model-invocation: true
---

# Review-fix

Close the loop on review findings and leave the trail on the PR. Every finding gets a verdict, every valid one gets its own fix commit, and every thread ends with a reply and a resolution a later reader can follow without this chat.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.

## 1. Gather findings

Resolve the topic and load `design.md` and `decisions.md`. Collect findings from `review.md` and from the PR's unresolved review threads:

```bash
gh api graphql -f query='{ repository(owner: "<owner>", name: "<repo>") {
  pullRequest(number: <n>) { reviewThreads(first: 100) { nodes {
    id isResolved path line comments(first: 20) { nodes { author { login } body } } } } } } }'
```

Other trackers: use the recipe `_shared/config.md` names.

## 2. Verify every finding

Against the actual code, with `decisions.md` open:

- **Valid.** A real issue. Note severity and the smallest safe fix.
- **Invalid.** Wrong, already handled, or a recorded deliberate deviation. Cite the decision entry or the code line.

Never fix an unverified finding. A decision entry the executing agent wrote mid-run is a claim to check against `design.md`, not an automatic dismissal.

## 3. Propose and wait

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before reporting. Present verified issues, their impact, and the proposed fix, blockers first; summarize rejected findings by reason. Write every verdict into `review.md`, log in `_state.md`, and commit the vault before waiting. That checkpoint approves nothing.

Wait for the human to approve the plan. They may drop, defer, or adjust items.

## 4. Implement

- One commit per fix: `fix(review): <finding> — <what changed>`.
- Execute's rules apply. Never weaken a test; a fix that changes behavior beyond its finding is drift, recorded or raised per the drift policy.
- Run the relevant tests. Nothing is pushed red.
- Fixes add no docs, screenshots, or scratch files to the repo. Evidence goes to the topic's `verification/` folder.

## 5. Leave the trail

After pushing:

- **Valid finding.** Reply on its thread with the one-line fix and the commit link, then resolve it (GraphQL `resolveReviewThread`).
- **Invalid finding.** Reply with the verdict and its citation, then resolve.
- **`review.md`-only finding.** Record the outcome in `review.md`.

Update `review.md` with every verdict and outcome. Log in `_state.md`; set phase `done` only when the human confirms nothing remains. Commit review outcomes, decisions, verification, and state together.

Reread the technical-writing skill before the final handoff. Say what was fixed, whether verification passed, and what still needs attention.

## When not to use

- `pr-review` produces the findings; this skill consumes them.
- `code-simplification` for cleanup no reviewer asked for.
