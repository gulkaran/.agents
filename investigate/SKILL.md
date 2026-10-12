---
name: investigate
description: Interactive investigation of an idea or issue. Explores the codebase and relevant sources, explains credible approach options, and ends with the human choosing the approach. Writes investigation.md to the topic folder. Use when starting work, revisiting an approach, or picking up an issue that isn't yet fully specified.
disable-model-invocation: true
---

# Investigate

Turn an idea or issue into understood territory and a **human-chosen
approach**. This phase is interactive: the human asks questions, redirects,
and makes the approach call. Automate the legwork, never the judgment.

Read first: `~/.agents/skills/_shared/config.md`,
`~/.agents/skills/_shared/topic.md`.
No production code changes in this phase.

## Process

### 1. Resolve the topic

Follow the topic protocol: resolve or create the topic folder and write the
`.topic` marker. For new work set phase `investigating` and log the session;
for an existing topic read its state first and use the reopening transition
when revisiting an approach. If the input is a tracker issue, fetch its full
body and comments via the tracker recipes and link it in `_state.md`.

### 2. Frame

Restate the idea in one or two lines: what we're trying to achieve and what
"done" would mean. If the goal itself is ambiguous, ask now — a short,
specific question, not a questionnaire.

### 3. Explore

Investigate the codebase and any relevant docs/sources until you can explain
where this work lives, what it touches, and what constrains it.

- **Stay single-agent by default.** Fan out subagents only when there are
  genuinely independent questions worth delegating (e.g. "how does system A
  handle X" and "what does library B support" have no bearing on each other).
  Delegated legwork can run on faster models; synthesis stays here.
- Ground every claim in the working tree, not memory or a search index.

### 4. Resolve uncertainties together

Use the exploration to separate facts you can establish from product and
business decisions that need the human's judgment. Answer factual questions
from the codebase and sources yourself; explain the evidence rather than
asking the human to look it up. Reuse goals and decisions already given.

Stress-test assumptions that could change the approach: who this serves,
which problem matters, what success means, scope, and acceptable trade-offs.
Treat these as a decision tree. Ask a focused round of questions whose
prerequisites are settled; save dependent questions for the next round.
For each question, give credible options when useful, your recommended
answer, and why. Be explicit about uncertainty behind the recommendation;
do not manufacture alternatives or turn a preference into an established fact.

Wait for the human's answers before settling those decisions. Continue
independent exploration while waiting, then use the answers to expose the
next relevant questions. Stop when the uncertainties that could materially
change the approach are resolved; leave detailed design choices for `design`.
If exploration already answers everything relevant, summarize that evidence
and proceed without inventing questions. These answers inform the approach;
they do not replace the human's approach choice below.

### 5. Synthesize findings

Identify the main findings, their practical impact, and how they affect the
approach. Preserve constraints, risks, prior art, and supporting evidence for
the artifact; distinguish decision-relevant findings from minor details.

### 6. Approaches

Present **credible alternatives when they exist** — with honest trade-offs
and a recommendation. **Never manufacture options**: if there is one obvious
way to do it, say exactly that and why, and present just it.

### 7. Report and converse

Read `~/.agents/skills/_shared/style.md` now, immediately before reporting.
Explain the main findings, intended features, and recommended approach in
simple terms, with a concrete example where useful. Then converse: answer
questions, dig deeper where asked, revise findings. Reread the style contract
before a revised report after further investigation. **The human picks the
approach.** Do not proceed on a recommendation alone.

### 8. Record

Before a handoff or waiting for an approach choice, save the current findings
and options in `investigation.md`, clearly marking the choice as pending.
Put supporting source research in `research/<subject>.md` and link to it.
Commit meaningful revisions using the shared topic protocol, including drafts.

On the human's choice:

- Update `investigation.md`: the human-facing explanation, complete findings and
  approach reasoning, the chosen approach, product/business answers and their
  rationale, any remaining uncertainties, plus an appendix (evidence, paths,
  commands run).
- Append the approach choice to `decisions.md` (decision + why).
- Update `_state.md`: phase → `designing`, log line.
- Commit the investigation, decisions, and state together. On a repeat
  investigation, revise the same file and explain why the approach changed.
- Tell the human the topic is ready for `design`.
