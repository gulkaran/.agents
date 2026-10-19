---
name: code-simplification
description: Simplify working code while preserving exact behavior. Use when the human asks to "simplify this", reduce complexity, or refactor for clarity. Keep simplification in its own commits.
disable-model-invocation: true
---

# Code simplification

Make working code easier to understand without changing its behavior.
The human invokes this when complexity needs attention. Execution and deslop
do not launch it automatically.

## 1. Understand the scope

Read repository instructions, the changed code, callers, tests, and relevant
history. Default to recently modified code. Follow neighboring conventions
rather than imposing a preferred style.

Identify responsibilities, public contracts, error paths, side effects, and
performance constraints. If you cannot explain why code exists, investigate
before removing it. Do not simplify already clear code to create activity.

## 2. Identify a concrete improvement

Look for nested control flow, unclear names, duplicated decisions, mixed
responsibilities, and abstractions that obscure what happens.

Choose the change that makes the code easier for another engineer to follow.
Line count and function length alone do not justify a refactor. Preserve a
helper when its name explains a domain concept.

Example:

> Replace a nested success path with early returns when the same checks run
> in the same order and produce the same errors. Do not reorder checks merely
> because the new order reads better.

Keep the scope within the requested area. A change to public interfaces,
storage, or behavior needs its own decision and implementation work.

## 3. Refactor incrementally

Start from committed, passing implementation so the simplification diff
stands alone. Make one coherent change, then run relevant checks.

Preserve outputs, data shapes, errors, side effects, evaluation order and
frequency, iteration semantics, and async boundaries. Avoid a simpler-looking
form that weakens error handling or materially worsens performance.

Existing behavior tests should pass without changed expectations. If they
fail, investigate and revert the simplification unless equivalence is
established. Do not weaken tests to justify the rewrite. Report a test coupled
to internals instead of silently altering its contract.

When coverage cannot establish a risky equivalence, add a focused behavior
test in a separate commit before refactoring, or leave that change alone.

## 4. Verify and commit

Review the before and after code for clarity and consistency. Run relevant
tests, build checks, and lint checks. Passing tests support the reasoning;
they do not prove every behavior stayed identical.

Commit simplification separately from features, fixes, tests, and deslop.
A focused simplification can stay in the same PR. A broad refactor needs
separate scope agreement and may warrant another PR.

Never amend, fixup, or squash simplification into another kind of commit during
PR preparation or review. Report the simplification commit links, what became
clearer, checks performed, and anything left unchanged because equivalence
was uncertain.

## When not to use

Use `deslop` for agent scratch artifacts and small redundant code patterns.
Use `review-fix` for findings from a review. Use implementation work for
behavior changes, even when a refactor exposes the bug.
