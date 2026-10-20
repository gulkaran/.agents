# Verification discovery and storage

## Running verification

Verify, browser use, and generated app verification skills do not ask whether
a task is work or personal. That question applies only to creating or
maintaining a reusable verification skill, as described below.

At verification time, use an app skill explicitly supplied by the caller or
referenced by the repository instructions. Otherwise look for matching
`verify-<app>` skills in the available skill catalog, the repository's
`.agents/skills/`, and `~/.agents/verification/work/<repo-key>/`. Read the matching
skill and check its applicability to the target app and revision. If several
remain ambiguous, ask which skill to use. If none exists, follow `verify`'s
generic discovery flow without creating a profile or requiring classification.

Use the caller's absolute evidence directory when supplied. Otherwise use an
external profile's configured evidence root, or default to
`~/.agents/verification/runs/<run-id>/`. Honor an explicitly configured evidence
location outside the application and shared skills repos. Do not select the
personal vault merely because a repo-local profile exists. Use distinct run and
PR directories so concurrent runs cannot overwrite evidence.

Pass the repository path, selected profile path if known, absolute evidence
directory, and tested revision to child workers. Keep credentials and browser
login state outside profiles, retained evidence, and commits.

## Creating or maintaining a verification skill

Only for `create-verification-skill` or `maintain-verification-skill`, ask before
inspection, tool calls, delegation, browser use, or writes:

**Is this work or personal?**

Wait for an explicit answer unless the human already supplied it for this run.
A suggested or preselected option is not an answer. Do not infer it from the
machine, repository owner, organization, remote, an old profile, or a previous
run. Children performing this creation or maintenance inherit the answer.
Keep it in the current run's handoff, not as a permanent classification.

An application profile is a `verify-<app>/` skill directory containing `SKILL.md`,
its feature map, and any maintained helpers.

| Context | Profile location | Evidence location |
|---|---|---|
| Personal | `<repo>/.agents/skills/verify-<app>/` | The resolved topic's `verification/` directory |
| Work | `~/.agents/verification/work/<repo-key>/verify-<app>/` | `~/.agents/verification/work/<repo-key>/evidence/<run-id>/` |

For work, the human may supply another external root for this run. Keep profiles
and evidence outside both the application repo and the synced skills repo.
Do not copy them into the personal vault or create an application PR for them.
The caller's general topic storage remains governed by its own workflow.

For personal creation or maintenance without a topic, use the configured vault's
`<project>/verification/<run-id>/`. Read `_shared/config.md` and its machine
override to resolve the vault. Do not create topic approvals for a standalone run.

Create or maintain profiles only in the chosen location. If several match the
target app, ask which one. Do not silently copy profiles between locations.
External profiles must record the application revision against which they were
proved. Recheck applicability on other branches.

## Repository identity and generated skills

Derive `repo-key` from the canonical remote host, owner, and repository path,
using nested directories and omitting credentials and `.git`. Worktrees use the
same remote identity. This locates profiles; it does not classify the task.
If creation or maintenance needs a key and no unambiguous identity exists, ask
for a stable project key. Ordinary verification can use generic discovery when
an external profile cannot be located.

Generated app skills must reference this protocol's running-verification rules,
including when invoked directly. They must not require the creation/maintenance
question to run. Browser-based app skills also read
`~/.agents/skills/browser/SKILL.md`. Their evidence instructions accept the
caller's resolved directory instead of hardcoding a machine-specific topic.
