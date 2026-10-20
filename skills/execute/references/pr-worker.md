# PR worker contract

The execute orchestrator launches one fresh worker per planned PR. The worker
builds only that PR and opens it as a draft. It does not verify the change in
a running application, clean the branch, or start the next PR.

## Input

The orchestrator passes the PR ID, branch, base, and the predecessor's head.
The first PR has no predecessor. Its base is the design's target branch.

Read `design.md` and `decisions.md` from `accio topic current --path`. The
design's PR plan gives this PR's scope, acceptance criteria, examples, and
test boundaries. Read the repository instructions for the test command and
any named pre-commit check.

## Work

1. **Branch.** Never discard or commit changes you did not make. For the
   first PR, fetch and fast-forward the local target branch to its remote
   tip, then run `gh stack init --base <base> <branch>`. For a later PR,
   check out the predecessor's branch at its handoff head and run
   `gh stack add <branch>`.
2. **Test first.** Read `../../tdd/SKILL.md`. Implement one behavior at a
   time. Write the test, watch it fail for the missing behavior, and commit
   it as red. Write only enough code to pass it and commit it as green. Take
   expected values from the design's examples, never from the code. Never
   weaken an assertion to pass. If you edit a test after its red commit,
   append the edit and its reason to `decisions.md`.
3. **Stay in scope.** Change production code only within this PR's scope in
   the design. Keep probe scripts, screenshots, and notes out of commits.
4. **Run the related tests.** Run the test files this branch adds or changes
   with the repository's documented test command. Add a lint or typecheck
   only when the repository instructions name one as a pre-commit check. Do
   not run the full suite, a browser, or the running application.
5. **Record deviations.** If the implementation differs from the design in a
   detail that does not change the contract, append the deviation and its
   reason to `decisions.md` and run
   `accio commit -m "record <pr-id> deviations"`. Otherwise write nothing to
   the vault.
6. **Open the draft PR.** Push the branch and follow
   `../../opening-a-pr/SKILL.md` with the assigned base and the draft option.
   The "How to test" section lists the tests you ran. Reuse an existing PR
   for this branch instead of opening a second one.
7. **Return the handoff.** Do not launch another worker or open another PR.

A blocker stops the worker: a related test that cannot pass without weakening
it, or material drift. Material drift is a changed acceptance criterion,
public contract, PR boundary, or a design assumption the code disproves.
Return the handoff with the blocker instead of working around it.

## Handoff

Return exactly these fields:

- PR ID
- Branch
- Base
- Head commit
- PR URL
- Tests run: the commands and their result
- Deviation recorded: yes with the `decisions.md` entry, or no
- Blocker: the facts, or none
