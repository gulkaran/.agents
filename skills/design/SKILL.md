---
name: design
description: Explain the technical changes, prove risky assumptions, and define an implementation contract for approval. Use after an approach is chosen or when revising an approved design.
disable-model-invocation: true
---

# Design

Explain how the system will implement the chosen behavior. Save the technical
contract in `design.md`, including storage, background work, and rollout
consequences, for human approval.

## 1. Check the chosen approach

Read `../_shared/config.md` and `../_shared/topic.md`. Resolve the topic and
verify brief and approach approvals. Return to the relevant phase for missing
checkpoints. Small tasks can use short documents but still need the decisions.

Read the brief, investigation, and decisions. Use the topic protocol's revision
transition before changing an approved design.

## 2. Test the riskiest assumption

Test the assumption most likely to invalidate the approach with a focused
experiment in `feasibility/`. Save its assumption, rerun instructions, result,
and limitations beside the files. Keep experiment code out of production.

If no experiment is needed, explain why. Preserve failed experiments and return
a disproven approach to investigation under the topic protocol.

## 3. Draft the technical contract

Write `design.md` with:

- Behavior and acceptance criteria linked to brief requirement IDs.
- Current and proposed flows, with a small diagram when useful.
- Changes to tables, migrations, APIs, workers, jobs, dependencies, permissions,
  and deployment order where applicable. Explain significant choices, their
  purpose, consequences, and alternatives.
- Data preservation, failure handling, and recovery where needed.
- The fewest coherent PRs, each with an ID, scope, dependencies, base strategy,
  and what works after it lands. Split for a review or rollout benefit.
- Verifiable implementation slices and test boundaries, including migration
  tests where needed. State what each boundary catches and misses.
- Feasibility evidence, unresolved risks, and deliberate deviations.

Stay within the chosen scope. Return newly necessary capabilities for a scope
decision. Describe mechanisms rather than listing technologies or writing
file-by-file pseudocode. Use existing codebase names and label proposed names
as new. For storage changes, name the tables and fields, their meanings,
relevant types or allowed values, and the code that writes and reads them.
Explain defaults, constraints, or indexes when they affect behavior or safety.

State explicitly when storage or background processing is unchanged.

## 4. Present the design for approval

Save and commit the draft. In the main chat presentation, explain how each
feature works, why the mechanism fits, and its data and operational consequences. Apply [technical-writing](../technical-writing/SKILL.md) for this explanation.

Use the concrete names and detail required above. The human must be able to
assess the implementation without opening `design.md`.

For example:

> Add new database field `browse_options.remote_scope` with values `country`, `region`, `local`,
> and `unknown`. The normalizer writes it so city searches can include
> country-wide remote jobs without reparsing descriptions on each request.
> A migration adds the field. A resumable startup rebuild classifies existing
> jobs from retained descriptions. This design pauses browsing during the
> rebuild to prevent mixed old and new results.

Distinguish schema changes from data rebuilds. Explain what happens to existing
data, how interrupted work recovers, and which rollout consequences follow from
the chosen implementation. Connect migrations and workers to the features they
support instead of listing them as separate warnings.

Include feasibility results and limits, the PR plan, test boundaries, and open
choices.

Revise until the human approves the contract and its test boundaries. Record
the checksum approval, set phase `design-approved`, and commit state and
decisions under the topic protocol.

## When not to use

Use `investigate` for an unsettled approach, `execute` for implementation, or
`to-tickets` to publish tracker issues when requested.
