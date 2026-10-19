# Workflow config

Read this before any workflow skill does anything. Every value here propagates
to all skills.

```yaml
vault_path: /Users/gulkaran/Desktop/vault/projects
tracker: github # one of: github, linear
scout:
  model: gpt-6.1-sol
  reasoning_effort: low
```

- `vault_path`: root under which topic folders live (see `topic.md`).
- `tracker`: which recipes file to follow for all issue-tracker operations:
  `~/.agents/skills/_shared/trackers/<tracker>.md`.
- `scout`: model and reasoning effort for read-only fact-finding subagents;
  see `scout.md`. Steps that decide stay on the session model.

## Per-machine override

If `~/.agents/skills/_shared/config.local.md` exists, its values override this
file. It is gitignored; use it on machines where the vault lives elsewhere.
