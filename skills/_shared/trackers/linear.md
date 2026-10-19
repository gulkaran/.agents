# Tracker recipes: Linear

How this setup expresses issue-tracker operations. All operations go through
the Linear MCP tools (or `linear` CLI if that's what's available). Linear has
native support for everything this workflow needs — use it, never body
conventions.

## Core operations

- **Create an issue:** create in the team the human works in; set title, body
  (markdown), and labels at creation.
- **Read an issue:** fetch the issue with its comments.
- **Comment / Close / Claim:** native comment; set state to Done/Canceled;
  assign yourself to claim.
- **Labels:** create the label once per workspace, then apply.

## Relationships

- **Parent/child:** native sub-issues (set `parentId`).
- **Blocking:** native "blocked by" relations — always, so the frontier is
  visible in Linear's own UI.
- Wire relationships in a second pass after creation (issues need IDs first).

## Frontier query (open + unblocked + unclaimed)

Filter: state is unstarted/backlog, no assignee, scoped by parent or label,
and no "blocked by" relation pointing at a non-completed issue.

## Referring to issues

In anything the human reads, refer to issues by **name wrapping the link**,
never a bare identifier like `ENG-142`.

## Wayfinding operations

- **The map** is an issue labelled `wayfinder:map`. Tickets are its
  sub-issues, each labelled `wayfinder:<type>`
  (`research`, `prototype`, `grilling`, `task`).
- **Claim** by assigning yourself before any work; open + unassigned means
  unclaimed.
- **Blocking** uses native relations (above).
- **Resolve**: post the answer as a resolution comment, mark Done, then append
  the one-line gist + link to the map's Decisions-so-far.
- **Frontier**: the frontier query above, scoped to the map's sub-issues.

## Triage label vocabulary

- `ready-for-agent` — ticket is fully specified; an agent can pick it up.
- `needs-human` — blocked on a human decision or manual task.
- Grouping: use Linear Projects/Milestones when the human asks for milestones;
  otherwise the blocking graph is the plan.
