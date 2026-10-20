# Example design report

This hypothetical notification design assumes the approved scope covers new
events only and the provider supports idempotency keys. Use the actual
system's names, constraints, and evidence when applying the skill.

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

## Explaining a backfill

When a backfill is necessary, explain it in the implementation section. For
example, if investigation established that failed events remain excluded from
automatic retry:

> Rescuing the timeout keeps future events eligible for retry. Existing events
> marked `failed` still fall outside the worker's query. We therefore need a
> backfill that returns confirmed timeout failures to `pending`. It excludes
> successful deliveries and unrelated failures. Changing the rescue alone would
> leave those existing notifications undelivered.
