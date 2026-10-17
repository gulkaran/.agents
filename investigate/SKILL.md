---
name: investigate
description: Research a confirmed brief and present an approach and complete scope for the human to choose. Use for "investigate this", exploring an implementation approach, or revisiting a topic after new evidence.
disable-model-invocation: true
---

# Investigate

Research the confirmed brief and present an approach for the human to choose.
Account for every requested capability. Make no production code changes.

## 1. Read the brief

Read `../_shared/config.md` and `../_shared/topic.md`. Resolve the topic and
check brief approval. If missing, use `../grill-me/SKILL.md` to prepare and
confirm it from existing context.

Read the brief, decisions, and linked issue comments. Reuse settled answers.
Use the topic protocol's revision transition for a changed goal.

## 2. Gather evidence

Group factual questions by code area or source. Launch read-only scouts under
`../_shared/scout.md` for independent questions, supplying relevant requirements,
unknowns, repository path, and questions.

Research current behavior, callers, persisted data, external contracts,
constraints, and prior art. Check claims against code or cited primary sources.
Use current online sources for external facts that matter.

Save findings in `research/<subject>.md`, separating facts, assumptions, and
what could not be checked.

## 3. Resolve new decisions

When evidence raises a product decision, explain the evidence and consequence,
then ask a focused question. Reuse the interview's answers; do not silently replace a
requirement. A constraint that changes the brief needs confirmation under the
topic protocol. Leave detailed implementation choices to design.

## 4. Propose the approach

Write `investigation.md` with current behavior, the recommendation, credible
alternatives, trade-offs, and uncertainties. Account for each brief requirement
as included, proposed for deferral, or unresolved. The human decides deferrals.
One credible approach is enough; do not invent alternatives.

Explain the change with a concrete example:

> Today, cancelling a delivery only hides it. The sender can still send it.
> Cancellation would prevent deliveries that have not started. Cancelling a delivery
> already in progress remains outside scope unless you choose otherwise.

Describe behavior to preserve, surprising consequences, and likely costs such
as a migration. Leave the technical contract to design.

## 5. Present and record the choice

Save and commit the report. Use `../wait-what/SKILL.md` to explain the proposal,
full scope, exclusions, and open choices in chat. Wait for the human's choice
of approach and scope.

Record the choice in `decisions.md` and the checksum approval in `_state.md`.
Set phase `designing` and commit the checkpoint under the topic protocol.

## When not to use

Use `grill-me` to establish intent, or `research` for a factual question that
does not choose or change the workflow's approach.
