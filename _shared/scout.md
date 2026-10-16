# Scout

A scout is a read-only subagent that answers a short list of factual
questions about one area of a codebase or a source and returns evidence per
question. It runs on the `scout` model in `config.md` because it reads and
reports; the caller decides.

## Launch

- One scout per area. Group the questions that share the same files, module,
  or source into one scout, three to five questions at most, so the reading
  is done once. Each agent pays a fixed cost to orient itself before it
  reads anything useful; twenty one-question scouts pay it twenty times.
- Split when questions touch different areas, or when one question is large
  enough to fill a context on its own. Launch the scouts in a single batch.
- Give it this file, the numbered questions, the repository or source path,
  and what counts as done.
- Set its model and reasoning effort from `scout` in
  `~/.agents/skills/_shared/config.md`, including the `config.local.md`
  override. Claude Code: the Agent tool's `model` field. Codex:
  `collaboration.spawn_agent` model settings. If the harness cannot set the
  model, say so and run the scout on the default model.

## Scout rules

- Read only: no edits, no commits, no commands with side effects.
- Ground every claim in the working tree or the named source. Cite
  `path:line` or the URL. Quote when the wording matters.
- Report facts, not recommendations. Separate found, not found, and could not
  check.
- Answer every question separately, in the order given. A question you
  could not answer gets "not found", never a guess folded into another
  answer.
- Stop when the questions are answered. Do not widen the search.

## Return format

Repeat this block once per question:

```
Question <n>: <as given>
Answer: <two to five sentences>
Evidence:
- path:line, what it shows
Not found / not checked: <one line each, or "none">
```
