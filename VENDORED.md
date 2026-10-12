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
