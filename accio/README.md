# accio

`accio` answers the vault and topic questions workflow skills ask mid-task:
where the vault is, which topic this checkout works on, where an artifact
goes, and how to commit the topic. It creates directories and markers and
leaves file contents to the agent.

## Install

```
cd ~/.agents/accio
just install
accio vault set ~/Desktop/vault
```

`just install` runs `cargo install --locked --path . --root ~/.local` with
`~/.cargo/bin` on `PATH`, so it works from shells that do not have cargo on
`PATH`, and puts the binary at `~/.local/bin/accio`. `vault set` is a one-time
step per machine; it records the vault root in the config file.

## Commands

| Command | Does | Example |
|---|---|---|
| `accio vault set <path>` | Records `<path>` as the vault after checking it is a directory and the root of a git repository; prints the canonical path. | `accio vault set ~/Desktop/vault` |
| `accio vault path` | Prints the configured vault root. | `ls "$(accio vault path)"` |
| `accio config get <key>` | Prints the value stored under `<key>`. | `accio config get tracker` |
| `accio config set <key> <value>` | Stores a string value; creates the config file when missing. `vault` is refused, use `vault set`. | `accio config set tracker github` |
| `accio config list [--json]` | Prints every `key: value` line sorted by key, or a JSON object. | `accio config list --json` |
| `accio topic use <slug> [--project <name>]` | Creates `<vault>/projects/<project>/topics/<slug>/` when missing, points this checkout's `.topic` marker at it, and prints `created <path>` or `using <path>`. | `accio topic use http-retry-budget` |
| `accio topic current [--path \| --json]` | Prints `project:`, `slug:`, `path:`, and `artifacts:` lines for the current topic; `--path` prints only the folder; `--json` prints an object. | `cd "$(accio topic current --path)"` |
| `accio topic list [--project <name>] [--json]` | Prints every topic in the vault as sorted `project/slug` lines, or a JSON array of `{"project","slug","path"}`. | `accio topic list --project infra-central` |
| `accio path <artifact> [--name <file> \| --next]` | Prints the absolute path of an artifact in the current topic and creates its directory when missing. | `accio path research --name evidence.md` |
| `accio commit -m "<message>"` | Stages and commits only the current topic folder in the vault repository as `docs(<slug>): <message>` and prints `<short sha> docs(<slug>): <message>`. Never pushes. | `accio commit -m "save design"` |

### `accio path`

The eight artifacts:

| Artifact | Kind | Path under the topic | `--name` | `--next` |
|---|---|---|---|---|
| `brief` | file | `brief.md` | no | no |
| `investigation` | file | `investigation.md` | no | no |
| `design` | file | `design.md` | no | no |
| `decisions` | file | `decisions.md` | no | no |
| `research` | directory | `research/` | yes | no |
| `feasibility` | directory | `feasibility/` | yes | no |
| `verification` | directory | `verification/` | yes | no |
| `review` | directory | `review/` | yes | yes |

A file artifact prints its path and creates nothing. A directory artifact
prints the directory and creates it. With `--name`, accio uses the entry
verbatim, adds no extension, and creates the printed path's parent, so
`--name screenshots/login.png` creates `screenshots/` under the artifact
directory. `--name` must be relative, non-empty, and free of `..` components.
`accio path review --next` prints `review/round-<n>.md` where `n` is one more
than the highest existing round (`round-<n>.md` with `n` a positive integer
without leading zeros), or `round-1.md`; accio does not fill gaps. `accio
path` never creates a file.

### `accio commit`

`commit` finds the vault repository from the topic folder, exits 1 with
`nothing changed in <project>/<slug>` when nothing under the folder changed,
then runs `git add -A -- <topic>` and `git commit -m "docs(<slug>): <message>"
-- <topic>`. Other staged work in the vault stays staged, hooks run as normal,
and a failing hook leaves the topic's changes staged for a rerun.

## Files accio owns

| File | Written by | Holds |
|---|---|---|
| `~/.config/accio/config.toml` | `vault set`, `config set` | One flat TOML table of string keys to string values. `vault` is the only key accio interprets. |
| `<toplevel>/.topic` | `topic use` | The absolute path of the topic folder for that checkout. Each worktree root has its own marker. |
| `<git common dir>/info/exclude` | `topic use` | One `/.topic` line, added once, shared by every worktree of the repository. |

Writes to the config file and the marker go through a temporary file and a
rename, so an interrupted write never truncates them.

## Vault layout

```
<vault>/projects/<project>/topics/<slug>/
  brief.md  investigation.md  design.md  decisions.md
  research/  feasibility/  verification/  review/round-<n>.md
```

The vault root is a git repository. `projects/` and `topics/` are fixed names.
accio creates topic folders and artifact directories and never writes a file
inside the vault.

## Resolution rule

`topic use`, `topic current`, `path`, and `commit` find the checkout with
`git rev-parse --show-toplevel` from the current directory and read
`<toplevel>/.topic`; outside a repository they exit 1 with `not inside a git
repository; run accio from a checkout`. The project and slug come from the
marker path's `projects/<project>/topics/<slug>` segments, so these commands
need no config file. `vault`, `config`, and `topic list` read only the config
file and work from any directory.

The default project for `topic use` is the repository name from the `origin`
URL with `.git` and leading dots removed (`gulkaran/.agents` gives `agents`),
or the checkout's directory name without `origin`; `--project` overrides both.
accio lowercases derived names. Slugs and project names must match
`^[a-z0-9]+(-[a-z0-9]+)*$`.

## Output and errors

stdout is plain text: one path per line, `key: value` lines for `topic
current` and `config list`, and `--json` where listed above. Runtime errors go
to stderr as `accio: <message>` with exit 1 and name the fix. Usage errors
(unknown verb, missing argument, conflicting flags) exit 2. accio never reads
stdin, never prompts, and never runs `git push`.

## Development

```
just test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Each verb is one variant in `src/cli.rs` and one `run(ctx, args)` in
`src/commands/<verb>.rs`. The domain lives in `src/{config,vault,topic,artifact}.rs`,
`src/git.rs` is the single git subprocess wrapper, and `src/atomic.rs` holds
the temporary-file-and-rename write. Integration tests under `tests/` drive
the binary against a temporary `HOME`, vault repository, and work repository.
