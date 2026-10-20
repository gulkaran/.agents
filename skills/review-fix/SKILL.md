---
name: review-fix
description: Verify review findings from the latest review round or PR review threads, propose the smallest safe fixes for approval, implement each as its own commit, then reply to and resolve every thread on the PR. Use after pr-review or when a PR has review comments to address.
disable-model-invocation: true
---

# Review-fix

Verify each finding, get approval for the fix plan, and commit each fix
separately. Reply to review threads with the outcome and leave unresolved
human decisions open.

## 1. Gather findings

Identify the target PR. Run `accio topic current --path` and read `design.md`,
`decisions.md`, and that PR's latest `review/round-<n>.md` in that folder,
then its unresolved review threads. Select the round for this PR, not another
stack member:

```bash
gh api graphql -f query='{ repository(owner: "<owner>", name: "<repo>") {
  pullRequest(number: <n>) { reviewThreads(first: 100) { nodes {
    id isResolved path line comments(first: 20) { nodes { author { login } body } } } } } } }'
```

For another tracker, follow `../_shared/trackers/<tracker>.md`, where
`<tracker>` is the value `accio config get tracker` prints.

## 2. Verify every finding

Against the actual code, with `decisions.md` open:

- **Valid.** A real issue. Note severity and the smallest safe fix: a code change or a deletion.
- **Invalid.** Wrong, already handled, or a recorded deliberate deviation. Cite the decision entry or the code line.
- **Needs the human.** Valid, but the only remedy is a new document, screenshot, record, or decision entry. Report it and leave it open. A post-red test edit with no justification is closed by reverting the edit or by the human accepting it, never by writing the entry now.

Never fix an unverified finding, and never create an artifact to close one. A decision entry the executing agent wrote mid-run is a claim to check against `design.md`, not an automatic dismissal.

## 3. Propose and wait

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before reporting.
Present verified issues, impact, and proposed fixes, with blockers first.
Summarize rejected findings by reason. Append every verdict under `## Verdicts`
in the selected round and commit with `accio commit -m "add review verdicts"`
before waiting.

Wait for the human to approve the plan. They may drop, defer, or adjust items.

## 4. Implement

- One commit per fix: `fix(review): <finding>, <what changed>`.
- Follow `../tdd/SKILL.md` for behavior fixes. Do not launch execute or rebuild the stack. Never weaken a test or silently change behavior beyond the approved finding.
- Run the relevant tests. Nothing is pushed red.
- Fixes add no docs, screenshots, or scratch files to the repo. Evidence goes to the directory `accio path verification --name <pr-id>` prints.
- Run deslop on the updated PR diff after fixes pass. Any cleanup gets its own commit. Preserve earlier cleanup and simplification commits; never fold them into a fix.

## 5. Record outcomes

After pushing:

- **Valid finding.** Reply on its thread with the one-line fix and the commit link, then resolve it (GraphQL `resolveReviewThread`).
- **Invalid finding.** Reply with the verdict and its citation, then resolve.
- **Needs the human.** Reply with what it needs from a person and leave the thread unresolved.
- **Round-file-only finding.** Record the outcome in the round file.

Append outcomes under `## Outcomes` in the selected round without changing
earlier rounds. Commit with `accio commit -m "add review outcomes"`.

Reread the technical-writing skill before the final handoff. Say what was fixed, whether verification passed, and what still needs attention.

## When not to use

- `pr-review` produces the findings; this skill consumes them.
- `code-simplification` for cleanup no reviewer asked for.
