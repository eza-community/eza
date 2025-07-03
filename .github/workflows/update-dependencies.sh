#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2025 Christina Sørensen
#
# SPDX-License-Identifier: EUPL-1.2

set -euo pipefail

commit_changes() {
  local file_to_check="$1"
  local commit_subject="$2"
  local commit_body="$3"

  # Check if the file has changes staged or unstaged
  if ! git diff --quiet --exit-code "$file_to_check"; then
    echo "$file_to_check has been updated. Committing changes."
    git add "$file_to_check"
    
    printf "%s\n\n%s" "$commit_subject" "$commit_body" | git commit -F -
  else
    echo "No changes to $file_to_check. Skipping commit."
  fi
}

BRANCH_NAME="deps_update_$(date --iso-8601)"
if git rev-parse --verify "$BRANCH_NAME" >/dev/null 2>&1; then
  echo "Branch '$BRANCH_NAME' already exists. Checking out."
  git switch "$BRANCH_NAME"
else
  git switch -c "$BRANCH_NAME"
fi

# 1. Update Cargo dependencies
echo "Checking for Cargo dependency updates..."
# Redirect stderr to stdout to capture cargo's output.
CARGO_OUTPUT=$(cargo update --recursive 2>&1)
UPDATED_CRATES=$(echo "$CARGO_OUTPUT" | grep 'Updating' || true)
commit_changes "Cargo.lock" "build(deps): cargo bump $(date --iso-8601)" "$UPDATED_CRATES"

# 2. Update Nix Flake dependencies
echo "Checking for Nix Flake dependency updates..."
# Use grep -A 2 to capture the 2 lines *after* the match.
FLAKE_OUTPUT=$(nix flake update 2>&1)
UPDATED_FLAKES=$(echo "$FLAKE_OUTPUT" | grep -A 2 'Updated input' || true)
commit_changes "flake.lock" "build(deps): flake bump $(date --iso-8601)" "$UPDATED_FLAKES"

echo "Dependency update process complete."
git status
