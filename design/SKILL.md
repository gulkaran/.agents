---
name: design
description: Turn a chosen approach into an approved implementation contract - feasibility proven, acceptance criteria set, PR stack planned. Formal human approval gate; writes design.md and stamps the approval. Use after investigate, or directly on a well-specified small task.
disable-model-invocation: true
---

# Design

Turn the chosen approach into an implementation contract the human formally
approves: what will be built, how we'll know it works, and in what PR-sized
slices. Execution refuses to start without this approval.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.
No production code changes — code written here is an exploratory spike,
retained as feasibility evidence rather than production implementation.

## Process

### 1. Resolve the topic

Follow the topic protocol. Expect phase `designing` with `investigation.md`
present. For a human-requested revision of an approved contract, first use the
shared revision transition. If there's no investigation (a small, well-specified
task), say so and ask whether to proceed directly; record the human's call in
the log.

### 2. Prove feasibility

Identify the **riskiest assumption** of the chosen approach — the thing that,
if false, invalidates the design. Prove or disprove it with the cheapest
possible spike (a scratch script, an API call, a 20-line prototype).

Before creating spike files, resolve the topic under the effective
`vault_path` (including `config.local.md` overrides). Create and work in
`<vault_path>/<project>/topics/<slug>/feasibility/`, not `/tmp` or another
temporary directory. Keep the spike's scripts, prototypes, inputs, and useful
outputs there so the work remains accessible after the session. Use descriptive
subfolders for separate experiments and preserve earlier evidence. If the vault
cannot be located or written, ask for the correct path rather than silently
falling back to temporary storage.

Record the assumption, how to rerun the spike, its result, and limitations
alongside the files. Summarize the evidence in the design's appendix and link
to the retained artifacts. Retain disproven experiments too. If nothing is
genuinely risky, say so explicitly — don't perform feasibility theatre or create
an empty folder. If the assumption is disproven, stop designing and use the
topic protocol's reopening transition: retain the failed evidence, record the
reason, invalidate any prior design approval, set phase `investigating`, and
commit. Return to the approach conversation and focused investigation; do not
continue implementation under the disproven approach.

### 3. Draft the design

- **Behavior contract:** acceptance criteria per slice — observable behavior,
  not implementation steps. These later become the tests execute writes first.
- **PR plan:** choose the fewest PRs that keep the work coherent and easy to
  review. Start with one PR for a cohesive feature. Split only at a natural
  boundary with a concrete review, dependency, rollout, or ownership benefit.
  Infrastructure, backend, and frontend can be useful boundaries when they
  stand on their own; do not split by layer automatically. Explain why each
  additional PR belongs on its own. A medium feature should not become five
  or six PRs without strong reasons.
- **Per-PR scope:** give each PR a descriptive title, a one-sentence
  responsibility, and a short, simple list of the concrete features or changes
  it adds. Say what works after it lands, including when it only lays groundwork
  for a later PR. Make dependencies and boundaries between PRs clear so the
  reader can tell which PR delivers each part of the feature. Labels like
  "backend" or "UI" alone are not enough.
- **Implementation slices:** use small, verifiable steps within those PRs.
  A session, test step, or ticket does not automatically need its own PR.
  Order genuine dependencies and record which slices belong to each PR;
  keep tests with the behavior they verify. Note delegation opportunities
  in the artifact, not as extra PR boundaries.
- **Test plan:** core behaviors to cover — major coverage of the concepts that
  matter, explicitly not exhaustive micro-testing.
- **Deliberate deviations:** anything knowingly outside standard practice,
  called out here and appended to `decisions.md` so review phases have the
  context.
- No file-by-file pseudo-code walkthroughs; the contract is behavior and
  slicing, not an implementation transcript.

### 4. The gate

Save `design.md` as a draft with its feasibility evidence before requesting
approval, and commit the related artifacts and state under the topic protocol.
Commit meaningful draft revisions too. If revising a previously approved
contract, clear its approval first; an old stamp never approves new behavior.

Read `~/.agents/skills/_shared/style.md` now, immediately before reporting.
Explain what will be built, how it will behave (with an example where useful),
why this approach fits, and any meaningful limits or trade-offs. Summarize the
feasibility result in plain language. Present the PR plan one PR at a time:
title, responsibility, a short, concise, simple list of features/changes, and
what works after it lands. Include dependencies or deferred behavior where
needed to make each PR's scope clear. Keep the full acceptance criteria and
technical evidence in `design.md`. Make every scope or behavior
choice needing approval clear in chat. Reread the style contract before
presenting a revised design after further work. Iterate until the human says
**"approved"** — explicitly. Denied or redirected means revise or return to
investigate using the shared reopening transition; never proceed on silence.

### 5. Record

On approval:

- Write `design.md` (design + feasibility evidence appendix).
- Update `_state.md`: `design_approved: <date>`, phase → `design-approved`,
  branch name if agreed; log line.
- Mark the design approved and commit the design, evidence, decisions, and
  state together using the topic protocol.
- If the human wants the stack on the tracker, publish it via the `to-tickets`
  skill (it reads the same tracker recipes) rather than reimplementing.
- Tell the human the topic is ready for `execute`.
