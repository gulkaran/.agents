# Skill style

How to write and edit a `SKILL.md` in this repo. Read it before every skill
edit. The reader is an agent that opens the file cold, mid-task, and acts on
what it says.

## Shape

- **Frontmatter.** `name`, a `description` that says when to use the skill and
  names trigger phrases, and `disable-model-invocation: true` when only the
  human should start it.
- **Opening paragraph.** What the skill produces and the one opinion that
  drives it. Two to four sentences.
- **Numbered steps.** One `## N. Verb phrase` per step, in the order the agent
  runs them.
- **Bullets.** Label, action, reason, in one or two sentences: "Commit red
  first: the Git history is the proof that tests preceded code."
- **Tables** for mappings. **Code blocks** for commands, file names, and
  output formats.
- **One example per non-obvious rule.** An example replaces several rules.
- **When not to use**, when a neighbor skill could be mistaken for this one.

## Budget

- About 500 words in `SKILL.md`. Move prompts, long lists, and output formats
  to `references/` and link them.
- A sentence of about 20 words. A paragraph of about 60.

## Editing

- Every added rule deletes or merges one, or names the failure it prevents in
  the same line.
- A rule that exists because of one bad run becomes a script or a hook, not a
  sentence.
- Write the skill as it should read today. Never narrate the change.
- Before saving, read the file as an agent that has never seen this repo.
  Strike every sentence that does not change an action.

## Words

Apply `~/.agents/skills/technical-writing/SKILL.md` and
`~/.agents/skills/unslop/SKILL.md`. Plain words, active voice, no em dashes,
no metaphors.
