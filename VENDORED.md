# Vendored skills

Seven skills are vendored as pinned copies from
[mattpocock/skills](https://github.com/mattpocock/skills).

- **Pinned commit:** `c55ee46073ed923f86ce59a5eb3b6d895095d1b7`
- **Skills:** `wayfinder`, `grilling`, `domain-modeling`, `research`,
  `prototype`, `to-spec`, `to-tickets`
  (`grilling` from `skills/productivity/`, the rest from `skills/engineering/`).

Wayfinder invokes `grilling`, `domain-modeling`, `research`, and `prototype`
**by name** for its ticket types — do not rename them. Our interactive
research skill is named `investigate` to avoid colliding with Matt's `research`
(AFK background doc-research).

## Local modifications

Kept deliberately minimal and additive; wayfinder's core behavior
(question-as-issue, resolution comment + close, hyperlinked ordering) is
untouched.

1. **wayfinder, to-spec, to-tickets:** replaced the
   `/setup-matt-pocock-skills` tracker-provisioning references with "read
   `~/.agents/skills/_shared/config.md` and follow the tracker recipes doc it
   names".
2. **to-tickets:** added an optional `## Context` section to the issue
   template linking the topic folder (see `_shared/topic.md`).
3. **research:** resolve output through the shared vault config and topic
   protocol before delegation; save supporting notes in the topic folder,
   preserve workflow synthesis and phase, and remove the repo-docs fallback.

To update the vendored set: re-copy from a newer commit, re-apply the
modifications above, and update the pinned SHA here.

## Cursor pstack skills

- **Upstream:** [cursor/plugins](https://github.com/cursor/plugins/tree/1e5c90a4b6f249917bd21fe361b3a2a7deb741b8/pstack/skills)
- **Pinned commit:** `1e5c90a4b6f249917bd21fe361b3a2a7deb741b8`
- **Skills:** `teach`, `how`, `why`, including their supporting references.
- **Local modifications:** added `agents/openai.yaml` metadata, preserving
  upstream explicit invocation; replaced Cursor agent/model configuration
  with Codex subagents that inherit the parent model; use available Codex
  tools for connector discovery and file reads, and the repository's
  configured PR tooling for history.

`teach` invokes `how`, `why`, and the existing `unslop` skill by name.
To update, copy the three folders from a newer upstream commit, reapply
these adaptations, and update the pinned commit here.
