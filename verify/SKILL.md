---
name: verify
description: Verify a change against its acceptance criteria using the real application, an app-specific verification skill when available, and generic discovery otherwise. Return results and screenshots for the PR description without editing product code.
---

# Verify an application change

First follow [the context gate](../_shared/verification.md). Ask work or personal
and wait before any inspection or tool call unless the caller passes the human's
answer for this run. Resolve profile and evidence storage only after that answer.

## Establish the scope

Read the supplied acceptance criteria, PR base and head, repository instructions,
and relevant existing tests. Confirm which checkout and build the running app
serves. Work from observable requirements rather than the implementer's claims.
Do not change product code, tests, or acceptance criteria, commit, or open a PR.
Return defects to the implementation worker for repair.

For a baseline-only assignment, capture relevant behavior before implementation,
record its revision, and return. The final verification pass must use the changed
revision. Do not present an old build's capture as proof of the current head.

## Load the app skill or discover the setup

If a profile exists in the chosen location, read its `SKILL.md`, feature index,
and feature files relevant to the change and its dependencies. Follow its launch,
doctor, drive, evidence, and cleanup instructions. Cover the affected entry points
the map lists. Report stale instructions without weakening expected behavior.

If none exists, use generic discovery for this run:

- Inspect documented startup commands, package scripts, services, migrations,
  seed data, authentication, and existing test tools.
- Determine the actual user interface and how to drive it. Read
  [browser](../browser/SKILL.md) for web or browser-based applications. For a CLI,
  API, mobile, or desktop application, use its existing appropriate tools.
- Confirm readiness and the intended build. Use isolated test state and own
  the resources the run starts.
- Derive journeys and expected results from the acceptance criteria. Exercise
  the real interface and verify relevant side effects.

Generic verification does not create a permanent app skill. Use
[create-verification-skill](../create-verification-skill/SKILL.md) when the human
requests a reusable profile, and
[maintain-verification-skill](../maintain-verification-skill/SKILL.md) for a full
profile audit. Do not run a full feature audit on every PR.

## Run and report

Run required checks, reusing passing results only for unchanged relevant content
and environment. Drive the affected journeys. For browser work, apply the shared
browser guidance even when an app profile exists. Inspect screenshots and check
requests, persistence, responsive layout, or keyboard behavior as the criteria
require. Capture useful diagnostics on failures. Cleanup must preserve evidence.

Return a concise report with:

- Tested revision, build, environment, profile used or generic discovery.
- Each acceptance criterion and its check, with passed, failed, blocked, or
  not-run outcome. Explain unavailable prerequisites and unverified boundaries.
- Relevant screenshot paths, captions, viewport, and whether each is before or
  after. State that the selected images were visually inspected.
- Defects, baseline issues, and evidence paths needed to reproduce failures.

Write the report and retained evidence only in the resolved evidence directory.
Do not retain routine passing logs. A failed or unavailable required check blocks
completion. Missing optional screenshots are a reported gap. Never call an
unreachable journey passed or manufacture evidence to close a gap.
