---
name: design
description: Explain the technical changes, prove risky assumptions, and define an implementation contract for approval. Use after an approach is chosen or when revising an approved design.
disable-model-invocation: true
---

# Design

Explain how to implement the approved approach in this codebase. Present the
design in chat, resolve the user's questions, and wait for approval. Then save
the full technical design in `design.md`. Make no production code changes.

## 1. Check the chosen approach

Read the approved investigation and relevant user decisions. If the approach
is unsettled, return to investigation. Read `../_shared/config.md` and its local
override when present for the handoff destination.

Inspect the affected code, persisted data, callers, and external contracts.
Establish how the system works today and where the proposed changes belong.
Use actual codebase names and cite relevant paths and lines. Label proposed
names as new.

## 2. Test the riskiest assumption

Use a small throwaway agent spike to test the assumption most likely to
invalidate the approach, or other uncertainties that could make it infeasible.
Give the agent a specific question and pass/fail criteria. Run the experiment
in an isolated temporary workspace. Keep its code out of production.

Record what the spike tested, the result, and its limits. Passing proves only
the tested capability. Design the production solution around the codebase's
architecture, without treating the spike's shortcuts as design decisions.

If existing evidence makes an experiment unnecessary, explain why. If the
spike disproves the approach, stop and report the evidence and failed
assumption. Propose returning to investigation. Do not silently substitute
another approach.

## 3. Work out the implementation

Settle the consequential implementation decisions before execution. Leave
ordinary coding details to the implementer. Use schemas or short pseudocode
when they clarify a mechanism, without writing a line-by-line coding script.

- Explain each substantial change, why it is necessary, and how it meets the
  approved requirements. Include enough existing behavior for someone unfamiliar
  with the system to understand the change.
- Describe transaction boundaries, APIs, workers, jobs, dependencies, permissions,
  failure handling, and recovery where relevant. Explain how the pieces interact.
- For storage changes, name the tables and fields, their purposes, types or
  allowed values, and readers and writers. Explain relevant defaults, constraints,
  and indexes. State why existing storage cannot meet the need.
- Distinguish schema migrations from changes to existing data. Explain which
  records need a backfill, why they need it, and why the forward fix alone does
  not handle them. Establish that need from evidence. Explain interrupted-work
  recovery and material deployment consequences.
- Plan verification against the acceptance criteria, including failure cases
  and migration checks where needed. Identify what each check establishes and
  what remains unverified.

Plan an ordered stack of PRs for `gh stack`, with the first based on the target
branch and each later PR based on its predecessor. Use the fewest coherent PRs.
Separate a migration only for a concrete review, compatibility, or deployment
reason. A separate PR does not itself require a separate deployment.

Stay within the approved approach and scope. Bring newly necessary capabilities
or changed requirements back to the user before continuing dependent design work.
Keep the proposed design in chat until approval.

## 4. What to report back

Write short sentences, name the actor, and explain technical terms when they
first matter. Apply [unslop](../unslop/SKILL.md). Present these in order:

1. Give one line stating what the feasibility spike tested and whether it passed.
   If no spike was needed, give the reason. Include a limit if omitting it would
   misrepresent the result. Passing is not a separate human approval checkpoint.
2. Explain the implementation by feature or substantial area of change. Describe
   how each part works and why it exists. When useful, walk through the sequence
   of events from the trigger to the result, showing where the changes occur.
   Use prose or numbered steps, not Mermaid diagrams. Explain every proposed
   migration, backfill, worker, or other substantial addition here.
3. Give a short ordered PR list mapping those already-explained changes to the
   `gh stack` plan. Introduce nothing new in this list. Keep detailed dependencies,
   verification, and rollout mechanics for the saved document unless they affect
   a decision the user needs to make now.

The user must be able to understand the proposed implementation and its material
trade-offs in chat. The PR list is a map of that explanation. Resolve open
choices, revise the proposal, and ask, "Do you approve this design?" Wait for
approval before writing `design.md`.

### Example report

This hypothetical notification design assumes the approved scope covers new
events only and the provider supports idempotency keys. Use the actual system's
names, constraints, and evidence when applying the skill.

> Spike passed. We confirmed that the Kafka handler can commit the order update
> and an outbox record in the same database transaction.
>
> Today, the Kafka handler updates an order, commits, and calls the notification
> provider. A crash after the commit can lose the notification. The new flow is:
>
> 1. The handler receives a Kafka event. In one transaction, it updates the order
>    and inserts a row in the new `notification_outbox` table. The row stores the
>    event ID, notification payload, and delivery state. The database commits
>    both changes or neither. The handler acknowledges the Kafka event only after
>    commit. A unique event ID lets it detect redelivery and skip the entire
>    transaction's effects for an event it has already processed.
> 2. A new worker claims due rows in a short transaction with row locks and
>    `SKIP LOCKED`, so concurrent workers skip rows another worker has locked.
>    It records a lease expiry and claim token, commits, then calls the provider.
>    The database transaction stays closed during the network call. Completion
>    updates must match the claim token, so an expired worker cannot overwrite
>    the result of a later claim.
> 3. Success marks the row sent. Temporary failures increment `attempts` and set
>    `next_attempt_at` with capped exponential backoff. Permanent failures or
>    exhausted retries mark the row failed for operator review. Expired leases
>    allow recovery after a worker crash. The worker reuses the event ID as the
>    provider's idempotency key so a retry after an uncertain response does not
>    send a second notification, within the provider's deduplication window.
>
> We need the outbox table because the current order record cannot represent
> pending delivery and retries. The migration creates this durable state before
> the handler starts writing it. Historical notifications are outside the agreed
> scope, so this design needs no backfill.
>
> PR 1 adds the outbox schema. PR 2 adds the handler transaction and the delivery
> worker, including claims, retries, and recovery described above. PR 2 is based
> on PR 1. We separate the schema because this service deploys migrations before
> application changes.

When a backfill is necessary, explain it in the implementation section. For
example, if investigation established that failed events remain excluded from
automatic retry:

> Rescuing the timeout keeps future events eligible for retry. Existing events
> marked `failed` still fall outside the worker's query. We therefore need a
> backfill that returns confirmed timeout failures to `pending`. It excludes
> successful deliveries and unrelated failures. Changing the rescue alone would
> leave those existing notifications undelivered.

## 5. Write the handoff

After approval, write `design.md` for the implementation agent. Include:

- The approved design and rationale, linked to investigation requirement IDs
  and acceptance criteria.
- Relevant existing behavior, proposed flows, code references, and concrete
  schemas, interfaces, transaction boundaries, and worker mechanics.
- Data changes, failure handling, recovery, and rollout details where applicable.
- The ordered PR plan with stable IDs, scope, dependencies, target and parent
  branches, and reasons for the boundaries. Map each PR to the explained changes.
- Verification for each slice, including acceptance criteria, failure cases,
  and relevant migration checks.
- Feasibility results, rerun details, limits, remaining risks, and accepted
  trade-offs. Distinguish confirmed decisions from inferences.

The document can contain more detail than the chat presentation. Expand the
approved decisions without introducing decisions the user never saw. If writing
reveals a new consequential choice, return to the user before recording it as
part of the approved design.

Use the destination supplied by the user or calling workflow. Otherwise use
`<vault_path>/<project>/topics/<slug>/design.md` from the shared config, with the
repository name as the project and a short topic slug. Reuse the known topic
folder when continuing existing work.

Do not load the topic-state protocol or manage its checksum approvals. Commit
only `design.md` in the Git repository containing it, preserving unrelated staged
changes. If the destination is not in a Git repository or the commit fails,
retain the document and report the missing commit.

Return the saved path and commit hash when available, then stop before implementation.

## When not to use

Use `investigate` for an unsettled approach, `execute` for implementation, or
`to-tickets` to publish tracker issues when requested.
