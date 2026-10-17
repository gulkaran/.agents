---
name: design
description: Explain the technical changes, prove risky assumptions, and define an implementation contract for approval. Use after an approach is chosen or when revising an approved design.
disable-model-invocation: true
---

# Design

Map the chosen behavior to the system changes that implement it.
Make storage, background work, and rollout consequences visible before the
human approves implementation. Save the contract in `design.md`.

## 1. Check the chosen approach

Read `../_shared/config.md` and `../_shared/topic.md`. Resolve the topic and
verify the brief and approach approvals. Missing checkpoints return to their
phase; a small task can have short documents, but still needs the decisions.

Read the brief, investigation, and decisions. Use the revision transition
before changing an approved design.

## 2. Prove the riskiest assumption

Identify the assumption that could invalidate the approach. Test it with a
focused experiment under the topic's `feasibility/` folder. Record the
assumption, rerun instructions, result, and limitations beside its files.
Spike code is evidence, not production implementation.

If nothing needs a spike, explain why. Preserve failed experiments. A disproven
approach returns to investigation through the topic protocol.

## 3. Draft the technical contract

Write `design.md` with:

- Behavior and acceptance criteria linked to the brief's requirement IDs.
- The current flow and proposed flow, with a small diagram when useful.
- Applicable changes to tables, migrations, APIs, workers, jobs, dependencies,
  permissions, and deployment order. Explain purpose, consequences, and
  alternatives for significant choices.
- Data preservation, failure handling, and recovery where the change needs them.
- The fewest coherent PRs, each with an ID, scope, dependencies, base strategy,
  and what works after it lands. Split for a concrete review or rollout benefit.
- Verifiable implementation slices and test boundaries. State what each
  boundary catches and misses, including migration tests when applicable.
- Feasibility evidence, unresolved risks, and deliberate deviations.

Keep scope within the chosen approach. A newly necessary capability goes back
for a scope decision. Avoid file-by-file pseudocode.

Example:

> We will store scheduled deliveries in a new table. A worker will claim
> deliveries when they become due. This needs a migration and a worker process.
> Retries can send a delivery twice, so the design must specify how duplicate
> sends are prevented.

Describe the mechanism chosen for this feature, not just a list of technologies.
Explicitly state when storage or background processing is unchanged, so absence
from the report is not mistaken for an oversight.

## 4. Present the design for approval

Save and commit the draft before asking. Read `../wait-what/SKILL.md`.
Explain what happens, how the main components implement it, the operational
consequences, feasibility results, and the PR plan. Make unresolved choices
explicit. Do not bury a migration or new worker in an appendix.

Revise until the human approves the technical contract. Record approval
against the document checksum, set phase `design-approved`, and commit state
and decisions. Approval includes the proposed test boundaries.

## When not to use

Use `investigate` when the approach is unsettled. Use `execute` to implement
the approved contract. Publish tracker issues through `to-tickets` if requested.
