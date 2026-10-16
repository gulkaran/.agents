---
name: investigate
description: Interactive investigation of an idea or issue. Scouts gather the facts, a grilling session settles the product and scope decisions with the human, and the human chooses the approach. Writes investigation.md to the topic folder. Use when starting work, revisiting an approach, or picking up an issue that isn't yet fully specified.
disable-model-invocation: true
---

# Investigate

Turn an idea or issue into a human-chosen approach. Scouts do the reading,
you do the synthesis, and the human makes every product and scope decision.
No production code changes in this phase.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.

## 1. Resolve the topic

Follow the topic protocol: resolve or create the topic folder and write the
`.topic` marker. New work: set phase `investigating` and log the session.
Existing topic: read `_state.md` first and use the reopening transition when
revisiting an approach. A tracker issue: fetch its body and comments with the
tracker recipes and link it in `_state.md`.

## 2. Frame

Restate the goal in one or two lines: what we are trying to achieve and what
"done" means. If the goal itself is ambiguous, ask one specific question now.

## 3. Send scouts

Split what you need to know into factual questions: where the work lives,
what it touches, the current user journey and its consumers, constraints,
prior art. Launch one scout per independent question in a single batch,
following `~/.agents/skills/_shared/scout.md`. Keep questions that need
judgment, or that depend on each other, for yourself.

Scouts read; they do not decide. Verify any scout claim you build on against
the working tree before it reaches the human.

## 4. Grill the human while scouts run

Run a grilling session, following `~/.agents/skills/grilling/SKILL.md`, on
the decisions only the human can make: who this serves, which problem
matters, what success means, scope, acceptable trade-offs. Ask the whole
frontier whose prerequisites are settled, each question with options and your
recommended answer. A question that depends on a scout still running waits
for that scout; the rest of the frontier does not.

- Facts are your job. Never ask the human for anything a scout can look up.
- Do not invent questions. If exploration answers everything, say so.
- Stop when nothing left could change the approach. Detailed design choices
  belong to `design`.

## 5. Synthesize and present approaches

Combine scout evidence and the human's answers into findings: current
behavior, proposed behavior, which consumers stay unchanged, and anything a
user would find surprising. Separate established requirements, your
recommendations, and open choices.

Present credible alternatives with trade-offs and a recommendation. One
obvious way: say so and present only it. Never manufacture options.

Read `~/.agents/skills/technical-writing/SKILL.md` immediately before
reporting, and again before any revised report. **The human picks the
approach.** A recommendation alone is not a choice.

## 6. Record

Before waiting for the approach choice, save `investigation.md` with the
choice marked pending and commit under the topic protocol. Put supporting
source research in `research/<subject>.md`.

On the human's choice:

- `investigation.md`: findings, every grilling round (question, recommended
  answer, the human's answer), the chosen approach and why, remaining
  uncertainties, and an appendix of evidence, paths, and commands.
- `decisions.md`: append the approach choice and its reasoning.
- `_state.md`: set phase `designing`, one log line.
- Commit all three together. On a repeat investigation, revise the same
  `investigation.md` and explain why the approach changed.
- Tell the human the topic is ready for `design`.
