---
name: research
description: Investigate a question against high-trust primary sources and capture the findings as a Markdown file in the configured knowledge vault. Use when the user wants a topic researched, docs or API facts gathered, or reading legwork delegated to a background agent.
---

Resolve the destination before delegating:

- Run `accio topic current --path`. If it prints a topic, use it. Otherwise run
  `accio topic use <slug>` with a short kebab-case slug for the research.
- Run `accio path research --name <subject>.md` for the output path. Reuse a
  matching note when continuing the same research; preserve unrelated notes and
  the workflow's synthesized `investigation.md`.
- If Accio cannot find the vault, ask the human to run `accio vault set <path>`.
  Do not fall back to repo docs or another convenient directory. Only an
  explicit user-specified destination overrides vault storage.

Spin up a **background agent** to do the research, so you keep working while it
reads. Pass it the research question and exact resolved output path. The
worker writes the note without committing; the coordinating agent owns the
vault commit after reviewing the result.

Its job:

1. Investigate the question against **primary sources** (official docs, source code, specs, first-party APIs), not a secondary write-up of them. Follow every claim back to the source that owns it.
2. Write the findings to a single Markdown file, citing each claim's source.
3. Save it to the supplied vault path and report the full path. Distinguish
   established facts from assumptions and unresolved questions.

After completion, commit the note with `accio commit -m "add research on <subject>"`,
including meaningful updates to an existing note. Research alone does not choose
an approach or advance the workflow.
