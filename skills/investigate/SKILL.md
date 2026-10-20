---
name: investigate
description: Research a feature or bug, establish requirements through questioning, and recommend an approach.
disable-model-invocation: true
---

# Investigate

Find out what to build and why. Research the problem, interview the user, and
recommend an approach. The interview is required even when the request looks
complete. Make no production code changes.

Leave implementation details to design. Research them here when they determine
whether an approach can meet the requirements.

## 1. Research the problem

- Run `accio topic use <slug>` with a short kebab-case slug for this work.
  Reuse the slug when revising an investigation.
- Establish current behavior, relevant code paths, callers, persisted data,
  external contracts, and constraints. Research prior art where useful. Check
  external claims against current primary sources and the project's versions.
- For a bug, trace the trigger through the code to the failure. Reproduce it when
  possible. Separate observations, facts established by the code, inferences,
  and what you could not check. Treat possible causes as hypotheses until the
  evidence supports them.
- Cite file paths and lines or source URLs.

Research enough to explain the problem and ask informed questions. Keep evidence
in chat until the final handoff. Recommend an approach after the interview and
targeted research.

## 2. Explain what you found

Give the user enough context to make decisions. Use ASD-STE100 Simplified
Technical English. Write short sentences, name the actor, and explain technical
terms when they first matter.

- For a bug, explain the issue, impact, and root cause or remaining hypotheses.
  Walk through one concrete failure, then show the code trace and evidence.
- For a feature, explain current behavior, the requested change, and the
  constraints the interview must resolve.

Example of a bug explanation:

> You cancel the delivery. The screen hides it, but the queued job still runs.
> The worker does not check whether the delivery was cancelled, so it sends it.

## 3. Ask the user questions

Interview the user until every decision within scope is settled. Organize
requirements, scope, and acceptable trade-offs as a tree of dependent decisions.

- For a feature, settle desired behavior, scope, UX, constraints, and acceptance criteria.
- For a bug, settle expected versus actual behavior, impact, acceptable fixes,
  behavior to preserve, and regression requirements.

Work in rounds:

1. Identify all decisions whose prerequisites are settled. This is the frontier.
   Pending answers and lookups block only the questions that depend on them.
2. Ask the whole frontier. Number each question, explain the choice, and give
   an explicit recommendation. Word it so "yes" accepts your recommendation
   and "yes to all" accepts the round.
3. Wait for the user's answers, then recompute the frontier. Reuse settled
   answers unless new evidence changes them.

Look up facts yourself. Put decisions to the user and wait. Ask independent
questions while lookups run.

Use this question format, including the emojis:

> ❓ **Q1. Cancellation scope.** Should cancellation prevent deliveries that have
> not started, while leaving deliveries already in progress outside this change?
>
> ➡️ Recommended: Yes. A delivery already being sent may be impossible to recall.

When every branch is settled, restate the agreed requirements, observable
acceptance criteria, exclusions, and accepted trade-offs in chat. Wait for the
user to confirm this shared understanding before proceeding. This confirmation
is required even for a fully specified request.

## 4. Check what the answers changed

Check which assumptions the answers changed and whether the evidence covers
every agreed requirement. Revise or discard approaches that no longer fit.

Investigate remaining gaps. If new evidence raises a decision or changes
requirements, explain it, return to the interview, and confirm the changed
requirements before proceeding.

## 5. Recommend an approach

Recommend the approach that best meets the confirmed requirements. The user
decides exclusions and deferrals. Resolve open choices and present these in
order, with enough detail to decide in chat.

- **Recap.** Briefly summarize the problem and agreed requirements. Expand on
  changed findings or details needed to assess the approach.
- **The approach.** What to build or change, why it meets every acceptance
  criterion, behavior to preserve, and material consequences such as a migration.
  Leave implementation details to design.
- **Alternatives and uncertainties.** Compare credible alternatives when they
  exist. Explain your preference, what remains unproven, and what design must resolve.
- **The decision.** Ask, "Do you approve this approach and scope?" Wait for approval
  separately from requirements confirmation. If the scope changes, revisit the
  affected questions and evidence before presenting a revised recommendation.

## 6. Write the handoff

After approval, write only `investigation.md` for the design agent. Include:

- The full problem explanation, including current behavior, who it affects,
  and a concrete example. For a bug, include the failure trace, cause, and how
  certain you are.
- Confirmed requirements with stable IDs, acceptance criteria, exclusions,
  and accepted trade-offs.
- The approved approach, rationale, behavior to preserve, and consequences.
- Relevant code paths, source citations, research, and reproduction findings.
- Rejected approaches and reasons, and open design questions.

Separate confirmed user decisions from your inferences. Add supporting detail
without introducing decisions the user never saw.

- Run `accio path investigation` and write `investigation.md` at the printed path.
- Commit with `accio commit -m "add investigation"`.
- Return the path and the commit line Accio prints, then stop before design.
