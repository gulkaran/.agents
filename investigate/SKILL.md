---
name: investigate
description: Research a confirmed brief and present an approach and complete scope for the human to choose. Use for "investigate this", exploring an implementation approach, or revisiting a topic after new evidence.
disable-model-invocation: true
---

# Investigate

Turn the confirmed brief into a researched approach the human chooses.
Account for every requested capability so scope cannot disappear during research.
Make no production code changes.

## 1. Read the brief

Read `../_shared/config.md` and `../_shared/topic.md`. Resolve the topic and
check its brief approval. If missing, use `../grill-me/SKILL.md` to prepare
and confirm the brief from existing context before continuing.

Read `brief.md`, existing decisions, and linked issue comments. Reuse settled
answers. A changed goal uses the topic protocol's revision transition.

## 2. Gather evidence

Split factual questions by code area or source. Launch read-only scouts under
`../_shared/scout.md` for independent questions. Give them the brief's relevant
requirements and unknowns, repository path, and specific questions.

Research current behavior, callers, persisted data, external contracts,
constraints, and prior art. Use primary online sources when external facts
matter. Verify claims you rely on against the code or cited source.

Save supporting findings in `research/<subject>.md`. Separate established
facts, assumptions, and things that could not be checked.

## 3. Resolve new decisions

Ask focused follow-up questions when evidence exposes a product decision.
Explain the evidence and consequence first. Do not restart the interview or
silently replace a requirement because it is inconvenient.

A new constraint that changes the brief needs the human's confirmation
through the topic protocol. Detailed implementation choices belong to design.

## 4. Propose the approach

Write `investigation.md` with the current behavior, recommended approach,
credible alternatives, trade-offs, and remaining uncertainties. Include a
capability list that accounts for each brief requirement as included,
proposed for deferral, or unresolved. A deferral needs the human's choice.

Use one concrete example to explain the change:

> Today, cancelling a delivery only hides it from the screen. The sender can
> still send it. The proposed approach makes cancellation prevent deliveries
> that have not started. Cancelling a delivery already in progress stays
> outside this proposal unless you choose otherwise.

Explain behavior to preserve and surprising consequences. Flag known costs
such as a likely migration, but leave the technical contract to design.
One credible approach is enough; do not invent alternatives.

## 5. Present and record the choice

Save and commit the pending report before asking. Read
`../wait-what/SKILL.md` and explain the proposal, full scope, exclusions, and
open choices in chat. The human chooses the approach and scope.

Record that choice in `decisions.md` and the approval in `_state.md`, bound
to the report's checksum. Set phase `designing` and commit the checkpoint.
A recommendation or research result cannot supply this approval.

## When not to use

Use `grill-me` to establish intent. Use `research` for a supporting factual
question that does not choose or change the workflow's approach.
