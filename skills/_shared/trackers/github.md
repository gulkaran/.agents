# Tracker recipes: GitHub Issues

How this setup expresses issue-tracker operations. All commands use `gh` in
the current repo. Prefer native relationships; fall back to the body
convention only where the repo/plan lacks the feature.

## Core operations

- **Create an issue:**
  `gh issue create --title "<name>" --body "<body>" --label <labels>`
- **Read an issue (body + comments):**
  `gh issue view <n> --comments`
- **Comment:** `gh issue comment <n> --body "<text>"`
- **Close:** `gh issue close <n>`
- **Claim (assign self):** `gh issue edit <n> --add-assignee @me`
- **Labels:** create once with `gh label create <name>`; apply with
  `gh issue edit <n> --add-label <name>`.

## Relationships

- **Parent/child (sub-issues):** native sub-issues via GraphQL:
  `gh api graphql` with the `addSubIssue` mutation (parent and child issue
  node IDs; get IDs via `gh issue view <n> --json id`).
- **Blocking:** native "blocked by" issue dependencies where available
  (GraphQL `addIssueDependency` / the Relationships panel). If the repo's plan
  lacks dependencies, fall back to a `Blocked by: #<n>` line at the top of the
  issue body — one line per blocker, kept current as blockers close.
- Wire relationships in a **second pass** after creating issues (issues need
  numbers before they can reference each other).

## Frontier query (open + unblocked + unclaimed)

`gh issue list --state open --no-assignee --label <scope-label> --json number,title,body`
then drop any issue whose blockers (native panel, or `Blocked by:` lines)
include a still-open issue: check with `gh issue view <blocker> --json state`.

## Referring to issues

In anything the human reads, refer to issues by **name wrapping the link**
(`[Title](url)`), never a bare `#42`.

## Wayfinding operations

- **The map** is an issue labelled `wayfinder:map`. Tickets are its
  sub-issues, each labelled `wayfinder:<type>`
  (`research`, `prototype`, `grilling`, `task`).
- **Claim** a ticket by assigning yourself before any work; an open,
  unassigned ticket is unclaimed.
- **Blocking** between tickets uses the Relationships recipes above.
- **Resolve** a ticket: post the answer as a resolution comment, close the
  issue, then append the one-line gist + link to the map's Decisions-so-far.
- **Frontier**: the frontier query above, scoped to the map's sub-issues.

## Triage label vocabulary

- `ready-for-agent` — ticket is fully specified; an agent can pick it up.
- `needs-human` — blocked on a human decision or manual task.
- Milestones: use GitHub Milestones if the human asks for grouping; otherwise
  the blocking graph is the plan.
