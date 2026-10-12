---
name: research
description: Investigate a question against high-trust primary sources and capture the findings as a Markdown file in the configured knowledge vault. Use when the user wants a topic researched, docs or API facts gathered, or reading legwork delegated to a background agent.
---

Read `~/.agents/skills/_shared/config.md` (including the
`config.local.md` override when present) and `~/.agents/skills/_shared/topic.md`
first. Resolve the destination before delegating:

- Use the topic protocol to locate an existing topic and read its `_state.md`.
  Confirm its folder is under the effective `vault_path`; if the marker points
  elsewhere, resolve that mismatch with the human rather than writing there.
- For a new research topic, use
  `<vault_path>/<project>/topics/<slug>/`, with the repo name as `<project>`
  unless specified otherwise. Create the topic and `.topic` marker according
  to the protocol, with phase `investigating`.
- Save supporting research as `research-<subject>.md` in that folder. Reuse a
  matching note when continuing the same research; preserve unrelated notes
  and the workflow's synthesized `research.md`.
- If the configured vault cannot be located or written, ask for the correct
  path or per-machine override. Do not fall back to repo docs or another
  convenient directory. Only an explicit user-specified destination overrides
  vault storage.

Spin up a **background agent** to do the research, so you keep working while it
reads. Pass it the research question and exact resolved output path.

Its job:

1. Investigate the question against **primary sources** (official docs, source code, specs, first-party APIs), not a secondary write-up of them. Follow every claim back to the source that owns it.
2. Write the findings to a single Markdown file, citing each claim's source.
3. Save it to the supplied vault path and report the full path. Distinguish
   established facts from assumptions and unresolved questions.

After completion, append a research session log line to the topic's
`_state.md`. Supporting research can run during any phase; preserve the
existing phase and approvals. Research completion alone does not choose an
approach or advance the workflow.
