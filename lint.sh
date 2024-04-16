#!/bin/bash
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

set -e

show_help() {
  echo "Validate Markdown in the godot-rust book."
  echo "Usage: $0 [fix]"
  echo ""
  echo "  fix:      Automatically apply fixes where possible"
  echo "  --help:   Show this help menu"
  echo ""
  echo "Requires markdownlint-cli2 to be installed. You can do so as follows:"
  echo "  npm install markdownlint-cli2 --global"
}

# If 'fix' is provided, append '--fix' to the end of the command
if [[ "$1" == "fix" ]]; then
  echo ">> Fix mode: apply fixes where possible."
  extra="--fix"
elif [[ "$1" == "help" ]] || [[ "$1" == "--help" ]]; then
  show_help
  exit 0
elif [[ -z "$1" ]]; then
  extra=""
else
  echo "Incorrect usage!"
  show_help
  exit 1
fi

# Could also use markdownlint-cli (command 'markdownlint') instead. Has more functionality, but the npm package is a bit heavier.

# shellcheck disable=SC2086
# do not quote $extra, it shouldn't be an argument if empty. 
# do quote glob pattern, shell expands differently than tool itself.
# keep in sync with CI arguments.
markdownlint-cli2 --config config/.markdownlint.jsonc ReadMe.md "src/**/*.md" $extra
