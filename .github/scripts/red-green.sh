#!/usr/bin/env bash
# Red-then-green check (SPEC.md section 10).
#
# Usage: red-green.sh <base-ref>
#
# Looks at the commits in <base-ref>..HEAD and checks that:
#   - the first `test:` commit fails `cargo test` (a compile error counts as failing),
#   - HEAD passes `cargo test`.
# Later `test:` commits (e.g. tests added for surviving mutants) may pass.
# PRs that change no `.rs` file are skipped.
#
# Needs a clean working tree; checks out commits and returns to the starting point.

set -euo pipefail

if [ $# -ne 1 ]; then
  echo "usage: $0 <base-ref>" >&2
  exit 64
fi
base=$1

if [ -n "$(git status --porcelain)" ]; then
  echo "::error::Working tree is not clean; refusing to check out other commits."
  exit 1
fi

head_sha=$(git rev-parse --verify HEAD)
# Remember the branch (or commit, if detached) so we can return to it.
start=$(git symbolic-ref --quiet --short HEAD || echo "$head_sha")
trap 'git checkout --quiet "$start"' EXIT

if ! git diff --no-ext-diff --no-textconv --name-only "$base...HEAD" -- '*.rs' | grep -q .; then
  echo "No .rs files changed; red-then-green check skipped."
  exit 0
fi

# Oldest first, so we pick the first tests commit.
red=""
while IFS=' ' read -r sha subject; do
  if [[ $subject =~ ^test(\([^\)]*\))?: ]]; then
    red=$sha
    echo "Tests commit: $sha $subject"
    break
  fi
done < <(git log --reverse --no-merges --format='%H %s' "$base..HEAD")

if [ -z "$red" ]; then
  echo "::error::No 'test:' commit found. Commit the tests on their own before the implementation."
  exit 1
fi

git checkout --quiet --detach "$red"
if cargo test --locked; then
  echo "::error::Tests commit $red passes 'cargo test'; it should fail before the implementation."
  exit 1
fi
echo "Tests commit fails, as expected."

git checkout --quiet --detach "$head_sha"
if ! cargo test --locked; then
  echo "::error::PR head $head_sha fails 'cargo test'."
  exit 1
fi
echo "Red-then-green check passed."
