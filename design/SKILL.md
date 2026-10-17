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
file-by-file pseudocode:

> A new table stores scheduled deliveries. A worker claims them when due.
> This requires a migration and a worker process. The design must prevent
> retries from sending a delivery twice.

State explicitly when storage or background processing is unchanged.

## 4. Present the design for approval

Save and commit the draft. Use `../wait-what/SKILL.md` to explain behavior,
components, operational consequences, feasibility results, and the PR plan.
Make unresolved choices explicit. Explain migrations and new workers in the
main presentation.

Revise until the human approves the contract and its test boundaries. Record
the checksum approval, set phase `design-approved`, and commit state and
decisions under the topic protocol.

## When not to use

Use `investigate` for an unsettled approach, `execute` for implementation, or
`to-tickets` to publish tracker issues when requested.
