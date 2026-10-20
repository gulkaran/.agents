---
name: design
description: Explain the technical changes, prove risky assumptions, and define an implementation contract.
disable-model-invocation: true
---

# Design

Explain how to implement the approved approach in this codebase. Present the
design in chat, resolve the user's questions, and wait for approval. Then save
the full technical design in `design.md`. Make no production code changes.

## 1. Check the chosen approach

Run `accio path investigation` and read the approved investigation and the
user's decisions. If Accio reports no topic, stop and tell the user to run
investigate first. If the approach is unsettled, return to investigation.

Inspect the affected code, persisted data, callers, and external contracts.
Establish how the system works today and where the changes belong. Use the
codebase's own names, cite paths and lines, and label proposed names as new.

## 2. Test the riskiest assumption

Launch a small throwaway agent to test the assumption most likely to invalidate
the approach, or another uncertainty that could make it infeasible. Give the
agent a specific question, pass and fail criteria, and an isolated temporary
workspace. Keep its code out of production.

Run `accio path feasibility --name <assumption>.md` and record what the spike
tested, the result, and its limits at the printed path. Cite that file from
`design.md`. A pass proves only the tested capability. Design the production
solution around the codebase's architecture, not the spike's shortcuts.

If existing evidence already settles the question, explain why instead of
running a spike. If the spike disproves the approach, stop. Report the evidence
and the failed assumption, and propose returning to investigation. Do not
substitute another approach.

## 3. Work out the implementation

Settle the consequential decisions before execution and leave ordinary coding
details to the implementer. Use a schema or short pseudocode when it clarifies
a mechanism, never a line-by-line script.

- **Changes.** Explain each substantial change, why it is necessary, and how
  it meets the approved requirements. Include enough existing behavior for a
  reader who does not know the system.
- **Mechanics.** Describe transaction boundaries, APIs, workers, jobs,
  dependencies, permissions, failure handling, and recovery where relevant, and
  how the pieces interact.
- **Storage.** Name the tables and fields, their purpose, types or allowed
  values, readers, and writers. Explain relevant defaults, constraints, and
  indexes, and why existing storage cannot meet the need.
- **Existing data.** Separate schema migrations from changes to existing
  records. Establish from evidence which records need a backfill and why the
  forward fix alone does not cover them. Explain interrupted-work recovery and
  material deployment consequences.
- **Verification.** Plan checks against the acceptance criteria, including
  failure cases and migration checks. State what each check establishes and
  what stays unverified.
- **PR stack.** Plan the fewest coherent PRs for `gh stack`, the first based on
  the target branch and each later PR on its predecessor. Separate a migration
  only for a concrete review, compatibility, or deployment reason. A separate
  PR does not itself require a separate deployment.

Stay within the approved approach and scope. If the design needs a new
capability or changes a requirement, bring it to the user before continuing
dependent work.

## 4. Present the design

Present the design in chat, in this order. Write short sentences, name the
actor, and explain a technical term when it first matters. Use ASD-STE100 Simplified
Technical English.

1. **Spike result.** Give one line stating what the spike tested and whether
   it passed, or why no spike was needed. Add a limit when omitting it would
   misrepresent the result. A pass is not a separate approval checkpoint, so
   do not pause for one.
2. **Implementation.** For each feature or substantial area of change, explain
   how it works and why it exists. When it helps, walk from the trigger to the
   result and show where the changes occur, in prose or numbered steps, not a
   Mermaid diagram. Every migration, backfill, worker, and other substantial
   addition belongs here.
3. **PR list.** Give a short ordered list mapping the changes above to the
   `gh stack` plan. Introduce nothing new. Leave dependency, verification, and
   rollout detail to `design.md` unless it affects a decision the user must
   make now.

The [example report](references/example-report.md) shows this order for a
notification outbox design. The user must be able to understand the proposed
implementation and its material trade-offs from the chat alone. Resolve open
choices, revise, then ask, "Do you approve this design?" Wait for approval
before writing `design.md`.

## 5. Write the handoff

After approval, write `design.md` for the implementation agent. Record the
decisions from step 3 in full, with code references and the existing behavior
they depend on. Also include:

- Links from each change to the investigation's requirement IDs and acceptance
  criteria.
- The PR plan with stable IDs, scope, dependencies, target and parent branches,
  and the reason for each boundary. Map each PR to the explained changes and
  give each its verification.
- Feasibility results with rerun details, limits, remaining risks, and accepted
  trade-offs. Mark which decisions the user confirmed and which are inferences.

The document may hold more detail than the chat, but no decision the user never
saw. If writing reveals a new consequential choice, return to the user before
recording it.

- Run `accio path design` and write `design.md` at the printed path.
- Commit with `accio commit -m "add design"`.
- Return the path and the commit line Accio prints, then stop before
  implementation.

## When not to use

Use `investigate` for an unsettled approach, `execute` for implementation, or
`to-tickets` to publish tracker issues when requested.
