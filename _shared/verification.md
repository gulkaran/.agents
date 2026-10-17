# Verification context and storage

## Ask before acting

Before repository inspection, topic resolution, tool calls, delegation, browser
use, or writes for a new verification workflow, ask the human:

**Is this work or personal?**

Wait for an explicit answer. A suggested or preselected option is not an answer.
Never infer the answer from the machine, repository owner, organization, remote,
an old profile, or a previous run. If the human explicitly supplied work or
personal in the current request, that satisfies the question.

Execute asks once at its entry point. Its implementation and verification
workers inherit that same human answer and do not ask again. Standalone browser,
verify, creation, and maintenance invocations must establish the answer first.
A worker without the answer returns to its caller before doing any work.
Keep the answer in the current run's handoff, not as a permanent classification.

## Resolve storage after the answer

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

Derive `repo-key` from the canonical remote host, owner, and repository path,
using nested directories and omitting credentials and `.git`. This identifies
the app; it never determines work or personal. Worktrees use the same remote
identity. If no unambiguous identity exists, ask for a stable project key.
Use a unique run ID so concurrent runs cannot overwrite evidence.

For standalone personal verification without a topic, use the configured vault's
`<project>/verification/<run-id>/`. Read `_shared/config.md` and its machine
override to resolve the vault. Do not create topic approvals for a standalone run.

Read only profiles in the chosen location. If several match the target app, ask
which one. Do not fall back from work to a personal profile or silently copy
profiles between locations. External profiles must record the application
revision against which they were proved. Recheck applicability on other branches.

Pass the human answer, repository path, selected profile path or absence,
absolute evidence directory, and tested revision to child workers. These are
resolved inputs, not settings for children to guess. Keep credentials and browser
login state outside profiles, retained evidence, and commits.

Generated app skills must begin by reading
`~/.agents/skills/_shared/verification.md` and honoring this gate, including when
invoked directly. Browser-based app skills also read
`~/.agents/skills/browser/SKILL.md`. Their evidence instructions accept the
caller's resolved directory instead of hardcoding a machine-specific topic.
