# CLAUDE.md

## Project

DiffLantern is a Rust CLI that shows the projected PR size (added + deleted lines) while you work.
`SPEC.md` is the source of truth. If code, tests and spec disagree, the spec wins. If the spec seems wrong or incomplete, stop and ask.

The owner is learning Rust and git through this project and is using it to test how far AI-written code can be trusted.

## Work in small steps

- At the start of every session, read `PROGRESS.md`, say which step is next and outline your plan. Wait for the owner to confirm before starting.
- Work on **one step from SPEC.md section 11 at a time**. Don't start the next step until the current PR is merged.
- Keep each PR well under 400 changed lines. If a step looks bigger, propose splitting it before writing code.
- Never work outside the current step's scope. Note unrelated problems in the PR description instead of fixing them.

## Test-driven loop (for every step)

1. List the SPEC.md scenarios the step covers. If a needed scenario is missing, propose it in plain English and wait for approval.
2. Write the tests only. Run them and confirm they **fail for the expected reason**. Commit: `test: <what>`.
3. Write the implementation. Run the full check (below) and confirm everything passes. Commit: `feat: <what>` (or `fix:` / `refactor:`).
4. Run `cargo mutants --in-diff` against the changes. For each surviving mutant, propose an English scenario first, then add the test.
5. In a final commit (`docs: update progress for step X.Y`), update `PROGRESS.md`: mark the step ✅, set **Next step**, and add a handover note (decisions, anything open, anything the next step needs). Only change the entries for the current step.
6. Write the PR description using the template below and hand over to the owner, who pushes and opens the PR.

### Never
- Weaken, delete or skip a test to make it pass. If a test seems wrong, stop and explain why.
- Add `#[mutants::skip]` or mutation exclusions without a written justification in the PR.
- Mark work as done when any check fails.

## Protected files (propose changes, don't make them silently)

`SPEC.md`, `CLAUDE.md`, the `TestRepo` helper, `.github/workflows/`, mutation testing config, and `Cargo.toml` dependencies.
Changes to these must be called out at the top of the PR description.

## Git

- **Commit locally only. Never push and never open PRs.** The owner reviews the commits, pushes and opens the PR.
- Never commit to `main`. Never rewrite history that has been pushed.
- One branch per step: `step/<number>-<short-name>`, e.g. `step/2.1-numstat-parser`.
- Before creating a branch, start from the latest `main`: run `git switch main`, then `git pull --ff-only`. Stop and ask if the working tree isn't clean, the pull fails, or anything else is unexpected.
- Small commits with conventional prefixes: `test:`, `feat:`, `fix:`, `refactor:`, `docs:`, `ci:`, `chore:`.
- Keep the tests-only commit separate from the implementation commit, so CI can verify red-then-green.

## Checks (all must pass before a PR)

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo mutants --in-diff <diff>
```

## Architecture rules

- **Pure core:** parsing, counting, status and formatting are pure functions that take text or data and return results. No I/O.
- **git only at the edge:** only the git runner module runs processes. The glue receives the runner as a parameter so tests can fake it.
- **Fakes only for failures:** use the fake runner only for cases real repos can't produce (git missing, too old, failing). Everything else uses real temp repos via `TestRepo`.
- **Read-only:** never fetch, never touch the index, never write config except on an explicit `--base` or `--limit`.

## Calling git safely

- Run git without a shell, with arguments as a list.
- Always pass `--no-ext-diff` and `--no-textconv` to diff commands, plus `-z` where supported.
- Validate branch names with `git check-ref-format`; put `--end-of-options` before user-supplied values.
- Strip control characters from anything printed that came from the repo.

## Testing rules

- Name tests descriptively (no scenario numbers) and group them in modules matching SPEC.md sections.
- Put the English scenario in a comment above each test.
- Tests must never read the developer's global or system git config.
- `TestRepo` actions verify their own effects.

## Dependencies

Ask before adding any crate. Say what it's for and why the standard library isn't enough.

## PR description template

```
## Step
<step number and name from SPEC.md>

## Protected files changed
<none, or list with reasons>

## Scenarios covered
<list>

## Evidence
- Tests failed before implementation: <yes, with failure reason>
- All checks pass: <yes>
- Mutation results: <caught / survived, and what was done about survivors>

## Rust concepts used
<short plain-English notes on any Rust features that appear for the first time>

## Open questions
<anything uncertain>
```

## When unsure

Stop and ask rather than guess. Say plainly when something is uncertain, untested or only partly done.
