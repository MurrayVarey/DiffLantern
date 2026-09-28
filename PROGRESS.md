# DiffLantern — Progress

Tracks the implementation steps in SPEC.md section 11.
Updated in the final commit of each step, so `main` only ever shows merged work.

**Status key:** ⬜ Not started · 🟨 In progress · ✅ Done · ⏭️ Merged into another step

**Next step:** 0.1

## Phase 0: Setup (owner)

| Step | Name | Status | PR |
|---|---|---|---|
| 0.1 | Rename repo; update local remote | ✅ | |
| 0.2 | Install Rust toolchain on Windows and Linux | ✅ | |
| 0.3 | Add README.md, SPEC.md, CLAUDE.md, PROGRESS.md | ✅ | |
| 0.4 | Ruleset on `main` and CODEOWNERS | ✅ | |

## Phase 1: Skeleton and safety net

| Step | Name | Status | PR |
|---|---|---|---|
| 1.1 | Cargo workspace (core library + CLI) with one trivial test | ⬜ | |
| 1.2 | CI: format, clippy, tests on Windows, Linux, macOS | ⬜ | |
| 1.3 | Mutation testing in CI | ⬜ | |
| 1.4 | Red-then-green commit check in CI | ⬜ | |
| 1.5 | Git runner | ⬜ | |
| 1.6 | Git version check | ⬜ | |
| 1.7 | `TestRepo` helper | ⬜ | |

## Phase 2: Core

| Step | Name | Status | PR |
|---|---|---|---|
| 2.1 | Parse `git diff --numstat -z` | ⬜ | |
| 2.2 | Tracked projected size | ⬜ | |
| 2.3 | Merge base behaviour | ⬜ | |
| 2.4 | Base errors | ⬜ | |
| 2.5 | Parse untracked file list | ⬜ | |
| 2.6 | Count untracked files | ⬜ | |
| 2.7 | Binaries | ⬜ | |
| 2.8 | Renames, permissions, line endings | ⬜ | |
| 2.9 | Read config | ⬜ | |
| 2.10 | Write config | ⬜ | |
| 2.11 | Status calculation | ⬜ | |
| 2.12 | Read-only guarantee | ⬜ | |

## Phase 3: CLI

| Step | Name | Status | PR |
|---|---|---|---|
| 3.1 | Argument parsing | ⬜ | |
| 3.2 | Human output and progress bar | ⬜ | |
| 3.3 | JSON output | ⬜ | |
| 3.4 | Exit codes, errors, end-to-end tests | ⬜ | |

## Phase 4: Oracle

| Step | Name | Status | PR |
|---|---|---|---|
| 4.1 | Test repo with real PRs | ⬜ | |
| 4.2 | Scheduled comparison job | ⬜ | |

## Phase 5: Release

| Step | Name | Status | PR |
|---|---|---|---|
| 5.1 | Release binaries for all three OSes | ⬜ | |
| 5.2 | Install instructions | ⬜ | |

## Handover notes

Newest first. One entry per completed step: decisions made, anything left open, and anything the next step needs to know.

<!--
### Step X.Y — Name (YYYY-MM-DD)
- Decisions:
- Open:
- For the next step:
-->
