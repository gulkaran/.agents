---
name: design
description: Turn a chosen approach into an implementation contract the human approves: riskiest assumption proven, acceptance criteria per slice, PR plan. Writes design.md and stamps the approval. Use after investigate, or directly on a small, well-specified task.
disable-model-invocation: true
---

# Design

Produce the contract execute builds against: what will be built, how we will know it works, and in which PR-sized slices. The riskiest assumption is tested before anything is promised, and nothing enters the contract that the chosen approach does not need. Execution refuses to start without the human's explicit approval.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.
Code written here is a spike kept as evidence, never production code.

## 1. Resolve the topic

Follow the topic protocol. Expect phase `designing` with `investigation.md` present. Revising an approved contract: use the shared revision transition first, so the old stamp cannot approve new behavior. No investigation (a small, well-specified task): say so, ask whether to proceed directly, and log the answer.

## 2. Prove the riskiest assumption

Name the one assumption that, if false, invalidates the approach. Prove or disprove it with the cheapest spike: a scratch script, an API call, a 20-line prototype.

- Work in `<vault_path>/<project>/topics/<slug>/feasibility/`, resolving `vault_path` with `config.local.md` overrides. Never `/tmp`. One subfolder per experiment; keep disproven ones. If the vault cannot be written, ask for the path.
- Record beside the files: the assumption, how to rerun, the result, limitations.
- Nothing genuinely risky: say so. No feasibility theatre, no empty folder.
- Disproven: stop designing. Use the topic protocol's reopening transition (retain evidence, record the reason in `decisions.md`, clear approval, phase `investigating`, commit) and return to the approach conversation.

## 3. Draft the contract

`design.md` holds, in this order:

- **Behavior contract.** Acceptance criteria per slice, as observable behavior. These become the tests execute writes first.
- **PR plan.** The fewest PRs that keep the work coherent. One PR for a cohesive feature; split only at a boundary with a concrete review, dependency, rollout, or ownership benefit, and name it. Layers (infra, backend, frontend) are not automatic boundaries.
- **Per-PR scope.** Title, one-sentence responsibility, a short list of what it adds, what works after it lands, dependencies on other PRs. "Backend" alone is not a scope.
- **Implementation slices.** Small verifiable steps inside those PRs, ordered by real dependency, tests kept with the behavior they verify. A session or ticket does not earn its own PR. Note delegation opportunities here, not as PR boundaries.
- **Test plan.** Core behaviors to cover; explicitly not exhaustive micro-testing.
- **Deliberate deviations.** Anything knowingly outside standard practice, also appended to `decisions.md`.

Scope stops at the chosen approach. No slice, option, migration path, or document the investigation did not ask for: extra slices are where over-building starts. No file-by-file pseudo-code.

## 4. Get explicit approval

Save `design.md` as a draft with its feasibility appendix and commit under the topic protocol before asking. Commit meaningful revisions too.

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before presenting, and again before a revised presentation. In chat: what will be built and how it behaves (an example where useful), why this approach fits, limits and trade-offs, the feasibility result in plain words, then the PR plan one PR at a time. Make every choice that needs approval explicit. Full criteria and evidence stay in `design.md`.

Iterate until the human says **approved**. Denied or redirected means revise, or return to investigate through the reopening transition. Silence is not approval.

## 5. Record

- `design.md`: mark approved, feasibility evidence as an appendix.
- `_state.md`: `design_approved: <date>`, phase `design-approved`, branch name if agreed, log line.
- Commit design, evidence, decisions, and state together.
- Tracker wanted: publish the stack with `to-tickets`; do not reimplement it.
- Tell the human the topic is ready for `execute`.
