# DiffLantern — Progress

Tracks the implementation steps in SPEC.md section 11.
Updated in the final commit of each step, so `main` only ever shows merged work.

**Status key:** ⬜ Not started · 🟨 In progress · ✅ Done · ⏭️ Merged into another step

**Next step:** 1.4

## Phase 0: Setup (owner)

| Step | Name | Status | PR |
|---|---|---|---|
| 0.1 | Rename repo; update local remote | ✅ | |
| 0.2 | Install Rust toolchain on Windows | ✅ | |
| 0.3 | Add README.md, SPEC.md, CLAUDE.md, PROGRESS.md | ✅ | |
| 0.4 | Ruleset on `main` and CODEOWNERS | ✅ | |

## Phase 1: Skeleton and safety net

| Step | Name | Status | PR |
|---|---|---|---|
| 1.1 | Cargo workspace (core library + CLI) with one trivial test | ✅ | |
| 1.2 | CI: format, clippy, tests on Windows, Linux, macOS | ✅ | |
| 1.3 | Mutation testing in CI | ✅ | |
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

### Step 1.3 — Mutation testing in CI (2026-09-29)
- Decisions:
  - Separate workflow, `.github/workflows/mutants.yml`, so the `Check (<os>)` job names from 1.2 are unchanged. Job name is `Mutants (diff)` on PRs and `Mutants (full)` on nightly (03:00 UTC) and manual (`workflow_dispatch`) runs.
  - Linux only: mutation results shouldn't depend on the OS, and `ci.yml` already covers all three.
  - cargo-mutants 27.1.0 is installed with `cargo install --locked` (no third-party action; costs a few minutes per run). If that gets annoying, cache the binary with GitHub's `actions/cache`.
  - PR diff is `git diff origin/<base>...HEAD` on a full-history checkout. The base branch name is passed through an env var, not inlined into the script.
  - Any missed or timed-out mutant fails the job (cargo-mutants exit code 2/3). `mutants.out/` is uploaded as an artifact on every run, pass or fail.
  - Each mode names its events (`pull_request` → diff; `schedule`/`workflow_dispatch` → full). Any other event fails the job, so adding a trigger forces a decision about which mode it gets.
- Open:
  - Only confirmed locally (empty diff, non-Rust diff, caught and missed mutants). It still needs to be seen running on the PR; the nightly and manual runs can only be checked once this is on `main`.
  - Not yet decided whether `Mutants (diff)` should be a required status check on `main`.
  - The SPEC.md stray text from step 1.1 is still there.
- For the next step:
  - 1.4 needs the PR's commit list, so it will also need `fetch-depth: 0` (as here).

### Step 1.2 — CI (2026-09-29)
- Decisions:
  - One workflow, `.github/workflows/ci.yml`, runs on every pull request and on pushes to `main`. A matrix job runs on `windows-latest`, `ubuntu-latest` and `macos-latest` with `fail-fast: false`, so every OS reports.
  - Rust comes from the runners' preinstalled `rustup` (latest `stable`, minimal profile plus rustfmt and clippy). The only third-party action is GitHub's own `actions/checkout`. The toolchain isn't pinned and there's no build caching.
  - Clippy and tests run with `--locked`, so a stale `Cargo.lock` fails CI. The workflow token is read-only (`contents: read`).
- Open:
  - This step is done only once all three runners are green on the PR. That's checked there, not locally.
  - An unpinned `stable` means a new Rust release could bring new clippy lints that fail CI without any code change. If that happens, consider a `rust-toolchain.toml`.
  - The SPEC.md stray text from step 1.1 is still there.
- For the next step:
  - Add mutation testing as a separate job or workflow. The job name `Check (<os>)` may be referenced by branch-protection rules.

### Step 1.1 — Cargo workspace (2026-09-28)
- Decisions:
  - Crates live under `crates/` (`difflantern-core` library, `difflantern` CLI); root `Cargo.toml` is a virtual workspace with `members = ["crates/*"]`, resolver 3, edition 2024, version/edition/license shared via `[workspace.package]`.
  - Core code is tested inside `difflantern-core`. cargo-mutants only runs the mutated crate's own tests by default, so a core function tested only from the CLI crate shows as MISSED. We chose this over `test_workspace = true` so broad CLI tests can't hide weak core unit tests.
  - `main()` is left empty until step 3.x; cargo-mutants generates no mutants for an empty `()` function.
- Open:
  - `SPEC.md` section 11, step 0.1 row has stray text ("This is done") that renders as an extra column.
- For the next step:
  - Local toolchain: cargo/rustc 1.98.1, cargo-mutants 27.1.0. Edition 2024 needs Rust ≥ 1.85 on CI runners.
  - `Cargo.lock` is committed (the workspace ships a binary).

<!--
### Step X.Y — Name (YYYY-MM-DD)
- Decisions:
- Open:
- For the next step:
-->
