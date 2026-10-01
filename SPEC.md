# DiffLantern — Specification

## 1. Purpose

Teams often cap pull requests at a fixed size (e.g. 400 changed lines). Git hosts only show the size once the PR exists, when it's often too late to split the work.

DiffLantern shows the **projected PR size** live, while you work, so you can see the limit coming.

## 2. Definitions

**Projected PR size**: the number of changed lines the PR would have if everything in the working tree were committed now.

- Measured from the **merge base** of the current branch and its configured **base branch**, to the **working tree**.
- Counted as **additions + deletions**, the same as GitHub. A changed line counts as one deletion plus one addition.
- **Includes:** committed changes on the branch, staged changes, unstaged changes, untracked files.
- **Excludes:** ignored files (`.gitignore`, `.git/info/exclude`) and binary files (as defined by git's own rules).

**Assumption:** DiffLantern projects the PR size as if everything in the working tree will be committed. This includes staged changes, unstaged changes and untracked files, but excludes ignored files and binaries. Where a file's staged and working versions differ, the working version is counted.

## 3. Principles

1. **Read-only.** DiffLantern never fetches, never touches the index and never changes repository state. The only exception is writing its own config, and only when the user explicitly sets a value.
2. **Match git and GitHub.** Use the real `git` executable, not a reimplementation, so numbers agree with what git and GitHub report.
3. **Show assumptions.** The base branch being measured against is always displayed.
4. **Freshness is the user's job.** Remote branches are only as fresh as the user's last `git fetch` or `git pull`.

## 4. Requirements

- git 2.30 or later (for `git diff --merge-base`).
- Windows and Linux (tested), macOS (tested in CI only).

## 5. Configuration

Stored in git config, written only through the `git config` command.

| Setting | Key | Default |
|---|---|---|
| Base branch for a branch | `branch.<name>.difflanternBase` | see resolution order below |
| Remembered default base | `difflantern.lastBase` | none |
| Line limit | `difflantern.limit` | 400 |

Storing the base under git's `branch.<name>` section means `git branch -d` removes it and `git branch -m` moves it automatically.

**Base branch resolution:**
1. `branch.<name>.difflanternBase`, if set.
2. `difflantern.lastBase`, if set.
3. The first of `origin/main`, `main`, `origin/master`, `master` that exists. *(Open decision)*
4. Otherwise: error asking the user to set a base.

**Detached HEAD:** use steps 2–4 and save nothing. *(Open decision)*

## 6. Phase 1 CLI

### Commands

| Command | Effect |
|---|---|
| `difflantern` | Print the projected PR size |
| `difflantern --help` | Print the DiffLantern commands |
| `difflantern --json` | Print the same data as JSON |
| `difflantern --base <branch>` | Save the base for this branch and as `lastBase`, then print |
| `difflantern --limit <n>` | Save the limit (positive integer), then print |

### Human output

```
+150 −60   210/400  █████░░░░░            vs origin/main
+230 −95   325/400  ████████░░  75 left   vs origin/main
+310 −120  430/400  ██████████▶ 30 over   vs origin/main
```

- **Under:** bar only.
- **Approaching:** bar plus "N left".
- **Over:** full bar with an overflow marker (`▶`) plus "N over". The bar never grows beyond its fixed width.
- **Colour:** default when under, amber when approaching, red when over. Meaning never relies on colour alone.
- Colour only when writing to a terminal, and never when the `NO_COLOR` environment variable is set.

### JSON output

```json
{
  "base": "origin/main",
  "additions": 230,
  "deletions": 95,
  "total": 325,
  "limit": 400,
  "status": "approaching",
  "left": 75,
  "over_by": 0
}
```

`left` is 0 when at or over the limit; `over_by` is 0 when at or under it.

### Status

| Status | Condition |
|---|---|
| `under` | total < 80% of limit |
| `approaching` | 80% of limit ≤ total ≤ limit *(at-limit allowed: open decision)* |
| `over` | total > limit |

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Under or approaching the limit |
| 1 | Over the limit |
| 2 | Error (not a repo, git missing or too old, base not found, invalid input) |

## 7. Open decisions

- [ ] **`.gitattributes` binary:** a text file marked `binary` is excluded (follows git's rule).
- [ ] **Renames:** a pure rename counts as 0, matching GitHub. Requires rename detection to be switched on explicitly, not left to the user's config.
- [ ] **Line endings:** converting a file between CRLF and LF counts every line as changed.
- [ ] **Default base order:** `origin/main`, `main`, `origin/master`, `master`.
- [ ] **Detached HEAD:** use remembered default and save nothing.
- [ ] **Exactly at the limit:** 400 of 400 is allowed (exit code 0).

## 8. Test scenarios

Base is `main` unless stated. Each scenario becomes one test, named descriptively and grouped by section.

### Committed changes
- Branch identical to `main` → +0 −0 = **0**
- Commit adds 5 lines to an existing file → +5 −0 = **5**
- Commit deletes 3 lines → +0 −3 = **3**
- Commit changes the text of 2 lines → +2 −2 = **4**
- Commit adds a new 10-line file → +10 −0 = **10**
- Commit deletes an 8-line file → +0 −8 = **8**

### Uncommitted changes
- Unstaged edit adds 4 lines → +4 −0 = **4**
- Staged edit adds 4 lines → +4 −0 = **4**
- Same file: 2 lines staged, then 3 more unstaged → +5 −0 = **5**
- Stage a 4-line addition, then remove 1 of those lines without staging → +3 −0 = **3**
- Commit adds 50 lines; an uncommitted edit removes 20 of them → +30 −0 = **30**
- Commit adds a line; an uncommitted edit removes it again → +0 −0 = **0**
- No commits on the branch; an uncommitted edit adds 6 lines → +6 −0 = **6**

### Untracked files
- Untracked 3-line file → +3 −0 = **3**
- Untracked 3-line file matching `.gitignore` → **0**
- Untracked 3-line file inside an ignored folder → **0**
- 3-line file ignored via `.git/info/exclude` → **0**
- Untracked 4-line file in a new subfolder → +4 −0 = **4**
- Untracked empty file → **0**
- Untracked 3-line file with no final newline → +3 −0 = **3**
- Untracked binary file → **0**
- Two untracked 2-line files, one with spaces and one with non-English characters in its name → +4 −0 = **4**

### Binaries
- Committed binary file, plus a 5-line text addition → +5 −0 = **5**
- Uncommitted change to a binary file → **0**
- 10-line text file marked `binary` in `.gitattributes` → **0** *(open decision)*

### Base branch and merge base
- Branch adds 5 lines; `main` then gains 20 lines of new commits → +5 −0 = **5**
- As above, then `main` is merged into the branch → +5 −0 = **5**
- Base branch doesn't exist → error naming the branch, exit code 2
- Branch shares no history with the base → error, exit code 2

### Other kinds of change
- File renamed with no edits → **0** *(open decision)*
- File renamed with 2 lines edited → +2 −2 = **4**
- Only a file permission changes → **0**
- 10-line file converted from CRLF to LF → +10 −10 = **20** *(open decision)*

### Configuration
- No config set → base is the first that exists of `origin/main`, `main`, `origin/master`, `master`
- None of those exist and nothing is configured → error asking to set a base, exit code 2
- Branch has a saved base → that base is used
- `--base develop` → saved for this branch and as `lastBase`
- New branch with no saved base, `lastBase` is `develop` → uses `develop`
- No limit set → 400
- `--limit 500` → saved and used
- `--limit 0`, `-5` or `abc` → error, exit code 2, config unchanged
- Detached HEAD → uses `lastBase` or default, saves nothing *(open decision)*
- Plain `difflantern` run → `.git/config` and the index are byte-for-byte identical afterwards

### Status and exit codes
- Total 319, limit 400 → `under`, exit code 0
- Total 320, limit 400 → `approaching`, exit code 0
- Total 400, limit 400 → `approaching`, exit code 0 *(open decision)*
- Total 401, limit 400 → `over`, exit code 1
- Run outside a git repo → error, exit code 2
- git missing, or older than 2.30 → clear error, exit code 2 (tested with a fake)

### Output
- Human output shows `+adds −dels`, total/limit, a progress bar and `vs <base>`
- Total 210, limit 400 → bar only, no "left" or "over"
- Total 325, limit 400 → shows "75 left"
- Total 430, limit 400 → full bar with `▶` and "30 over"
- Output piped (not a terminal) or `NO_COLOR` set → no colour codes
- `--json` includes base, additions, deletions, total, limit, status, left and over_by

## 9. Testing strategy

| Layer | What | How |
|---|---|---|
| Unit | Parsing, status, formatting | Pure functions fed captured git output. No git, no mocks. |
| Integration | All repo scenarios | Real throwaway repos via the `TestRepo` helper, with git config isolated from the machine. |
| Fake git | git missing, too old, failing | A fake "run git" function passed into the glue code. |
| CLI | Output, JSON, exit codes | Run the real binary. |
| Mutation | Test strength | `cargo-mutants`: changed code on each PR, full run nightly. |
| Oracle | Agreement with GitHub | A test repo with real PRs; a scheduled job compares GitHub's numbers with DiffLantern's. |

**`TestRepo` helper rules:**
- Every action verifies its own effect (e.g. `stage` confirms the file is staged).
- Every repo is created in a temp folder and deleted automatically.
- Tests never read the developer's global or system git config.

## 10. Workflow and safeguards

### The loop for every step
1. Confirm the step's scenarios (section 8) are right.
2. Write the tests. Commit them on their own and confirm they **fail** for the expected reason.
3. Write the implementation. Commit; CI shows them **passing**.
4. Run `cargo-mutants` on the change. For each surviving mutant, add an English scenario first, then a test.
5. Open a PR (under the line limit). Review it, optionally with a fresh Claude session as a second reviewer. Merge.

### Enforced by GitHub, not by instructions
- **Branch protection on `main`:** no direct pushes; passing checks and owner approval required.
- **CODEOWNERS:** owner review required for `SPEC.md`, test scenarios, the `TestRepo` helper, `.github/workflows/` and mutation config.
- **Skip markers:** any new `#[mutants::skip]` needs a written justification in the PR.

## 11. Implementation steps

Each step is one small PR. **Done when** means all its checks pass in CI on Windows, Linux and macOS.

### Phase 0: Setup
| Step | Goal | Done when |
|---|---|---|
| 0.1 | Rename repo to DiffLantern; update local remote | `git remote -v` shows the new URL | This is done
| 0.2 | Install Rust (`rustup`, `rustfmt`, `clippy`) on Windows and Linux | `cargo --version` works on both |
| 0.3 | Add `README.md`, `SPEC.md`, `CLAUDE.md` (workflow instructions) | Merged via PR |
| 0.4 | Branch protection on `main` and CODEOWNERS | A direct push to `main` is rejected |

### Phase 1: Skeleton and safety net
| Step | Goal | Done when |
|---|---|---|
| 1.1 | Cargo workspace: `difflantern-core` (library), `difflantern` (CLI); one trivial test | `cargo test` passes locally |
| 1.2 | CI: format check, clippy and tests on Windows, Linux and macOS | All three runners green |
| 1.3 | Mutation testing in CI (`--in-diff` on PRs, nightly full run) | Job runs and reports |
| 1.4 | ~~Red-then-green commit check in CI~~ Dropped: mutation testing covers test strength; tests-first is a working habit, not a CI check | — |
| 1.5 | Git runner: run git, return stdout, stderr and exit code | Test runs `git --version` |
| 1.6 | Git version check (≥ 2.30): pure parser plus fake-git tests | git missing and too-old scenarios pass |
| 1.7 | `TestRepo` helper with self-verifying actions and isolated config | Helper's own tests pass |

### Phase 2: Core
| Step | Goal | Scenarios |
|---|---|---|
| 2.1 | Parse `git diff --numstat -z` output (pure), incl. binary marker and rename format | Unit tests on captured output |
| 2.2 | Tracked projected size: merge base to working tree | Committed changes; uncommitted changes |
| 2.3 | Merge base behaviour | Base branch and merge base (first two) |
| 2.4 | Base errors | Base missing; no shared history |
| 2.5 | Parse `git ls-files --others --exclude-standard -z` (pure) | Unit tests on captured output |
| 2.6 | Count untracked files | Untracked files |
| 2.7 | Binaries in tracked and untracked files | Binaries |
| 2.8 | Renames, permissions, line endings | Other kinds of change |
| 2.9 | Read config: limit (default, validation), base resolution, detached HEAD | Configuration (read cases) |
| 2.10 | Write config: `--base`, `--limit` | Configuration (write cases) |
| 2.11 | Status calculation (pure) | Status scenarios |
| 2.12 | Read-only guarantee | Config and index unchanged |

### Phase 3: CLI
| Step | Goal | Scenarios |
|---|---|---|
| 3.1 | Argument parsing | `--json`, `--base`, `--limit` accepted and validated |
| 3.2 | Human output and progress bar (pure formatting) | Human output |
| 3.3 | JSON output | JSON output |
| 3.4 | Exit codes and error messages; end-to-end binary tests | Status and exit codes |

### Phase 4: Oracle
| Step | Goal | Done when |
|---|---|---|
| 4.1 | Public test repo with a set of real PRs covering the scenarios | PRs exist on GitHub |
| 4.2 | Scheduled job comparing GitHub's numbers with DiffLantern's | Job runs; any mismatch fails it |

### Phase 5: Release
| Step | Goal | Done when |
|---|---|---|
| 5.1 | Build binaries for Windows, Linux and macOS on tagged releases | Binaries attached to a GitHub release |
| 5.2 | Install instructions, incl. `git-difflantern` naming for `git difflantern` | README updated |

## 12. Later

- `--watch`: file watching with debounce; performance targets (update within 2 s of a save, under 1% CPU when idle)
- PR size check in DiffLantern's own CI, using DiffLantern
- VS Code extension (using `--json`)
- GUI
- `difflantern prune`: remove config for deleted branches
- Stacked-branch hint ("looks like this may be based on feature-a")
- Option to count additions only
- Team-wide settings file committed to the repo
