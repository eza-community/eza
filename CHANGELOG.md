<!--
SPDX-FileCopyrightText: 2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# Changelog

## [0.23.0] - 2025-07-18

### Bug Fixes

- [**breaking**] Make --grid work when not in TTY
- [**breaking**] Stdin behavior

### Documentation

- Add `--smart-group` option to README

### Features

- Add icons for changelog and todo files
- Use CHANGES icon for CHANGELOG as well

### Miscellaneous Tasks

- Remove unused dependency

## [0.22.1] - 2025-07-12

### Bug Fixes

- Replace default_input_path check with "." check

### Documentation

- "cheks" should be "checks"

### Features

- [**breaking**] Define -d/--treat-dirs-as-files behavior, tests
- Refresh icon set with new glyphs and additions

### Miscellaneous Tasks

- Eza v0.22.0 changelogs, version bump
- Eza v0.22.1 changelogs, version bump

### Build

- Bump phf from 0.11.3 to 0.12.1
- Cargo bump 2025-07-03
- Flake bump 2025-07-03
- Bump windows-sys from 0.59.0 to 0.60.2

### Ci

- Automate dependency updates

## [0.21.6] - 2025-06-26

### Documentation

- Add missing --absolute option to man page

### Features

- Add prettier icon for `.prettierrc.{json,json5,toml,yaml,yml}`

### Miscellaneous Tasks

- Upgrade FreeBSD to 14.3-RELEASE in unit tests workflow
- Eza v0.21.6 changelogs, version bump

### Build

- Bump libc from 0.2.172 to 0.2.174
- Cargo bump 2025-06-26
- Flake bump 2025-06-26

## [0.21.5] - 2025-06-20

### Bug Fixes

- Use OpenBSD 7.7 that has MSRV, instead of 7.6
- Excessive open file descriptors
- Typo
- Impl desirable behaviour for unreadables
- Unreadables format style
- Clippy warnings
- Missing word in comment

### Miscellaneous Tasks

- Eza v0.21.5 changelogs, version bump

### Styling

- Fix various clippy warnings for rust 1.86
- Remove unnecessary semicolon

## [0.21.4] - 2025-05-30

### Bug Fixes

- Escape spaces in file path to make them work correctly
- List inside working dir with `--list-dirs` and no path passed
- Ignore incorrect Unicode path instead of crashing on Windows

### Miscellaneous Tasks

- Add Visual Studio icon for *.suo
- Add swift icon for *.xcplayground
- Add dropbox icon
- Eza v0.21.4 changelogs, version bump

### Refactor

- Clippy lints

### Styling

- Update formatting

### Build

- Flake bump 2025-05-29
- Cargo bump 2025-05-29
- 1.81 -> 1.82

## [0.21.3] - 2025-05-02

### Features

- Add support for .ipynb file icons

### Miscellaneous Tasks

- Eza v0.21.3 changelogs, version bump

### Styling

- Unbreak formatting

### Build

- Bump DeterminateSystems/nix-installer-action from 16 to 17
- Cargo bump 2025-05-01
- Flake bump 2025-05-01

## [0.21.2] - 2025-04-25

### Bug Fixes

- Make clippy work again

### Documentation

- Update Fedora install instructions for Fedora 42
- Specify perf improvements

### Features

- Remove dependency on once_cell

### Miscellaneous Tasks

- Add icon for '.stowrc' files
- Eza v0.21.2 changelogs, version bump

### Performance

- Use a hashmap when possible for file extension matching

### Styling

- Clean up glob matching code

### Build

- Bump uutils_term_grid from 0.6.0 to 0.7.0

## [0.21.1] - 2025-04-19

### Bug Fixes

- Don’t truncate branch name
- Hi extension icon wasnt working as it was in the wrong aray

### Documentation

- Update README.md
- Add crates.io link for README.md badge

### Miscellaneous Tasks

- Add MS DOS icon for *.com
- Add ruby icon for config.ru, Gemfile, Gemfile.lock, procfile, rake, rakefile and change ruby icon
- Add python icon for *.pxd and *.pyx
- Add markdown icon for *.mdx
- Add fsharp icon for *.f# and *.fsscript
- Add clojure icon for *.cljc and *.edn
- Eza v0.21.1 changelogs, version bump

### Build

- Flake bump 2025-04-19
- Cargo bump 2025-04-19

## [0.21.0] - 2025-03-31

### Bug Fixes

- Flake bump 2025-03-20
- Remove unnescesarry unsafe blocks for libc major/minor device id
- Unwrap -> expect on libc deviceid calls
- Formatting issue
- Fix unused PermissionsPlus fields

### Miscellaneous Tasks

- Eza v0.21.0 changelogs, version bump

### Build

- Cargo deps 2025-03-20
- [**breaking**] Change MSRV 1.78.0 -> 1.81.0
- Bump flake deps 2025-03-30
- Bump cargo deps 2025-03-30

## [0.20.24] - 2025-03-13

### Bug Fixes

- Make temp files visible on white background

### Documentation

- More precise temp files color description

### Features

- Add `.exercism` folder icon
- Add `.ocamlinit` icon
- Add `.opam` folder icon

### Miscellaneous Tasks

- Add gcloud icon for .gcloudignore
- Add vim icon for .gvimrc, _vimrc and _gvimrc
- Add fennel icon for ~/.fennelrc and ~/.config/fennel/fennelrc
- Eza v0.20.24 changelogs, version bump

### Build

- Bump once_cell from 1.20.3 to 1.21.0
- Bump terminal_size from 0.4.1 to 0.4.2
- Bump serde from 1.0.218 to 1.0.219
- Bump chrono from 0.4.39 to 0.4.40

## [0.20.23] - 2025-02-27

### Bug Fixes

- Add Pixi installation instructions

### Miscellaneous Tasks

- Eza v0.20.23 changelogs, version bump

### Build

- Bump libc from 0.2.169 to 0.2.170
- Bump serde from 1.0.217 to 1.0.218
- Bump log from 0.4.25 to 0.4.26
- Bump trycmd from 0.15.8 to 0.15.9

### Ci

- Remove magic nix cache

## [0.20.22] - 2025-02-20

### Features

- Add prettier icon for *.prettierignore
- Add icon for *.hrl
- Add photoshop icon for *.psb
- Add eslint icon for .eslintignore
- Add renovate icon for renovate.json
- Add elixir icon for *.eex, *.leex and mix.lock

### Miscellaneous Tasks

- Eza v0.20.22 changelogs, version bump

### Build

- Bump once_cell from 1.20.2 to 1.20.3

## [0.20.21] - 2025-02-13

### Bug Fixes

- Start publishing libgit arm builds

### Miscellaneous Tasks

- Eza v0.20.21 changelogs, version bump

### Build

- Add libgit alternative for arm builds

## [0.20.20] - 2025-02-07

### Bug Fixes

- Make `flake.lock` icon the nix logo

### Miscellaneous Tasks

- Eza v0.20.20 changelogs, version bump

## [0.20.19] - 2025-01-30

### Bug Fixes

- Update MSRV to 1.78 to solve rust malfunction
- Rustc false dead code positives
- Rustc false positives in tests
- Regression in theme config location, simplify path
- Wrong file name for Brewfile

### Documentation

- Add note regarding ci msrv update

### Features

- Add `.norg` icon

### Miscellaneous Tasks

- Eza v0.20.19 changelogs, version bump

### Build

- Bump dirs from 5.0.1 to 6.0.0
- Bump NexusPHP/no-merge-commits from 2.1.0 to 2.2.1
- Bump flake
- Bump cargo

### Ci

- Fix and unify msrv and add matrix to free/netbsd

## [0.20.18] - 2025-01-23

### Bug Fixes

- Support additional yaml file extension, clippy

### Miscellaneous Tasks

- Eza v0.20.18 changelogs, version bump

### Build

- Cargo bump 2025-01-23
- Flake bump 2025-01-23

### Ci

- Use rust 1.74 instead of latest via rustup

## [0.20.17] - 2025-01-16

### Features

- Add editorconfig icon for .editorconfig

### Miscellaneous Tasks

- Eza v0.20.17 changelogs, version bump

### Build

- Cargo bump 2025-01-16
- Flake bump 2025-01-16

## [0.20.16] - 2025-01-09

### Features

- Add brew icon for brewfile and brewfile.lock.json

### Miscellaneous Tasks

- Eza v0.20.16 changelogs, version bump

### Build

- Update flake inputs 2025-01-08
- Update cargo inputs 2025-01-08
- Bump git2 from 0.19.0 to 0.20.0

## [0.20.15] - 2025-01-02

### Features

- Add icons from nerd fonts 3.3.0 release & more
- Add new icons, extensive list

### Miscellaneous Tasks

- Eza v0.20.15 changelogs, version bump

### Build

- We switch to our own fork of natord

### Ci

- Bump FreeBSD / NetBSD versions.

## [0.20.14] - 2024-12-26

### Bug Fixes

- ...those pesky workflow targets
- Remove separate bsd tests
- Remove audit workflow

### Features

- Audit checks in main CI

### Miscellaneous Tasks

- Eza v0.20.14 changelogs, version bump

### Refactor

- Move eza, trycmd packages
- Move BSD unit tests to main flow

### Styling

- Format workflows
- Ci checks formatted

### Build

- Cargo 2024-12-25

### Ci

- Simplify
- Let's just always run em'
- Only run big checks on PRs to main
- Faster flake checks... maybe?
- Flakes on latest ubuntu
- Only do no-merge-commits on PR

## [0.20.13] - 2024-12-18

### Bug Fixes

- Pre-commit-hooks.nix trying to be too clever
- Remove stray description (originally from `--decay-mode`)

### Miscellaneous Tasks

- Eza v0.20.13 changelogs, version bump

### Build

- Update crate deps Mon Dec 16
- Update flake deps Mon Dec 16

### Ci

- Update to PRESENT DAY, PRESENT TIME

## [0.20.12] - 2024-12-11

### Bug Fixes

- Add unicode-3.0 license
- Use safe terminal_size_of
- Use terminal_size_of with borrowed raw handle

### Features

- Move MSRV to 1.74 and deep bump cargo deps
- Add Gleam lang icon

### Miscellaneous Tasks

- Eza v0.20.12 changelogs, version bump

### Testing

- Regen for 1.74

### Build

- Bump terminal_size from 0.3.0 to 0.4.1

### Ci

- Openbsd 7.4 -> 7.6

## [0.20.11] - 2024-12-05

### Bug Fixes

- Bump libc from 0.2.165 to 0.2.167

### Miscellaneous Tasks

- Eza v0.20.11 changelogs, version bump

## [0.20.10] - 2024-11-28

### Bug Fixes

- People dislike the phrasing "maintained" on hackernews

### Miscellaneous Tasks

- Eza v0.20.10 changelogs, version bump

### Build

- Bump libc from 0.2.164 to 0.2.165

## [0.20.9] - 2024-11-21

### Bug Fixes

- Remove newline after doc comment of `regen` recipe

### Miscellaneous Tasks

- Eza v0.20.9 changelogs, version bump

### Refactor

- List all recipes by default
- Group related recipes

### Build

- Bump libc from 0.2.162 to 0.2.164
- Bump DeterminateSystems/nix-installer-action from 15 to 16

## [0.20.8] - 2024-11-14

### Bug Fixes

- Cross-compiling by updating to libz-sys to 1.1.20

### Miscellaneous Tasks

- Eza v0.20.8 changelogs, version bump

### Build

- Bump palette from 0.7.5 to 0.7.6
- Bump libc from 0.2.161 to 0.2.162
- Bump serde from 1.0.214 to 1.0.215

## [0.20.7] - 2024-11-07

### Bug Fixes

- Palette v0.7.6 -> v0.7.5

### Miscellaneous Tasks

- Update package.exclude list in Cargo.toml
- Eza v0.20.7 changelogs, version bump

### Build

- Bump DeterminateSystems/nix-installer-action from 14 to 15
- Bump serde_norway from 0.9.38 to 0.9.39
- Bump trycmd from 0.15.7 to 0.15.8

## [0.20.6] - 2024-10-31

### Bug Fixes

- Changelog spelling

### Documentation

- Fix typo `--get-repos-no-status` to `--git-repos-no-status`

### Miscellaneous Tasks

- Eza v0.20.6 changelogs, version bump

### Build

- Bump serde from 1.0.210 to 1.0.214

## [0.20.5] - 2024-10-25

### Bug Fixes

- Ensure nested tree parts align under item name
- Remove depricated `chrono` `from_timestamp_opt`

### Miscellaneous Tasks

- Update generated test files
- Eza v0.20.5 changelogs, version bump

### Build

- Bump libc from 0.2.159 to 0.2.161
- Chrono v0.4.34 -> v0.4.38

## [0.20.4] - 2024-10-18

### Bug Fixes

- Filetype, coloring for executables and folder

### Miscellaneous Tasks

- Eza v0.20.4 changelogs, version bump

## [0.20.3] - 2024-10-17

### Bug Fixes

- Git cliff docs issue

### Miscellaneous Tasks

- Eza v0.20.3 changelogs, version bump

### Performance

- Reuse filetype from DirEntry

## [0.20.2] - 2024-10-09

### Bug Fixes

- Colors in old ms command prompt
- Bring help text in line with available flags
- Do not print parent filename with --absolute=on

### Miscellaneous Tasks

- Add fox installation option
- Eza v0.20.2 changelogs, version bump

### Build

- Bump once_cell from 1.20.1 to 1.20.2

## [0.20.1] - 2024-10-03

### Bug Fixes

- Release recipe
- Support passing multiple options for generate-trycmd-test.sh
- Move options into flags
- Rustfmt errors

### Documentation

- Fix cross-references
- Update file type colors
- Document that exit 13 == permission denied

### Features

- Update just, add more formats
- Recursively walk symlinks pointing at dirs
- Add --follow-symlinks option
- Add autocomplete for --follow-symlinks
- Show directories last

### Miscellaneous Tasks

- Eza v0.20.1 changelogs, version bump

### Testing

- Add cases for -T and --follow-symlinks
- Regenerate tests broken by line number changes

### Build

- Fix manual version
- Bump once_cell from 1.19.0 to 1.20.1

### Ci

- Remove flakehub, flakestry publish

## [0.20.0] - 2024-09-26

### Bug Fixes

- Flake trycmd bug
- Pre-commit-hook taplo bug

### Documentation

- Add link to eza-themes repository in readme
- Cargo install dir inaccurate
- Add x-cmd method to install eza
- Adding a testing infos file to guide everyone through tests

### Features

- Add `opml` file extension
- Add a regen rule
- [**breaking**] Relicensed to EUPL-1.2

### Miscellaneous Tasks

- Eza v0.20.0 changelogs, version bump

### Refactor

- Move some files to `.config`
- Release scripts use `.config`
- Relicense to EUPL-1.2

### Styling

- Switch to nixfmt rfc style, format tree
- Remove blank line

### Testing

- Regenerate integration tests
- Regenerate tests

### Build

- Darwin devShell resuse eza deps
- Ensure flake inputs aren't duplicated'
- Remove semnix deps
- Bump flake lock 2024-09-26
- Removed unused flake follows
- Add cargo to devShell
- Add clippy to devShell
- Use toolchain in devShell
- Bump libc from 0.2.158 to 0.2.159
- Bump unicode-width from 0.1.13 to 0.2.0

### Ci

- Full nix3 command output in logs
- Allow EUPL-1.2
- Unblock windows

## [0.19.4] - 2024-09-18

### Bug Fixes

- Remove non_alpha from percent encoding to fix hyprlinks

### Features

- Pass from serde_yaml to serde_norway

### Miscellaneous Tasks

- Eza v0.19.4 changelogs, version bump

## [0.19.3] - 2024-09-12

### Bug Fixes

- Convert empty space to %20 when render hyperlinks
- Split commit workflows and run no-merge-commits only on PRs
- Correct naming of commit related workflows

### Documentation

- Better version bump commit summary

### Features

- Add no-merge-commits job to commits workflow

### Miscellaneous Tasks

- Rename justfile
- Eza v0.19.3 changelogs, version bump

### Refactor

- Rename conventional-commits workflow to commits

### Build

- Bump DeterminateSystems/nix-installer-action from 13 to 14
- Bump DeterminateSystems/flake-checker-action from 8 to 9
- Bump actions/checkout from 3 to 4
- Bump libc from 0.2.155 to 0.2.158
- Bump nu-ansi-term from 0.50.0 to 0.50.1

## [0.19.2] - 2024-09-05

### Bug Fixes

- Remove unnecessary map and make clippy happy
- Adjust grid details for CI tests
- Imports and merge conflicts
- Rustfmt issues
- Clippy issues
- Revise UiStyles::plain to have no style at all
- Pr reviews fixes for theme file
- Selectively filter files when recursing #1101
- Fix typo in FromOverride<FileKinds> impl
- Add serde(default) to StyleOverride.foreground/background fields

### Documentation

- Add Flox to INSTALL.md
- Add ic for icon color to colors man page
- Add further documentation about theme file

### Features

- Add c++ module interfaces as source file types
- Add icon field to UiStyles
- Add ic key for UiStyles icon in set_exa
- Add None as icon value in UiStyles.default_theme
- Add icon function to FileNameColours trait
- Implement FileNameColours.icon for Theme
- Adjust FileName.paint to consider possible icon color
- Begin implementation of config file
- Allow writing default theme.yml file for eventual config file implementation
- Theme file configuration base
- Add IconOverrides struct and UiStyles.icon_overrides
- Add icon_override function to FileNameColours trait
- Implement FileNameColours.icon_override for Theme
- Handle icon overrides in FileName.paint
- Add example config for icon_overrides
- Rename UiStyles.icon_override to icons and add Style field
- Add shorthand aliases to StyleOverride variables
- Add custom deserialize_color and use in StyleOverride
- Outsource color_from_str function to make it testable

### Miscellaneous Tasks

- Release eza v0.19.2

### Refactor

- Simplify icon style setting in FileName.paint
- Make every setting optional with override layer
- Simplify sample theme.yml
- Formatting for rustfmt macro

### Styling

- Fix clippy issue in FileName.paint
- Apply rustfmt
- Simplify from_str_radix calls to please clippy

### Testing

- Add unit tests for color_from_str function

### Build

- Bump windows-sys from 0.52.0 to 0.59.0

### Ci

- Allow MPL-2.0

## [0.19.1] - 2024-08-28

### Bug Fixes

- FreeBSD build.
- Typo

### Miscellaneous Tasks

- Release eza v0.19.1

### Build

- Bump uzers from 0.12.0 to 0.12.1

## [0.19.0] - 2024-08-08

### Bug Fixes

- [**breaking**] Implement `EZA_GRID_ROWS` grid details view minimum rows threshold

### Miscellaneous Tasks

- Release eza v0.19.0

## [0.18.24] - 2024-08-03

### Bug Fixes

- 1.80 breakage from time crate

### Miscellaneous Tasks

- Release eza v0.18.24

### Build

- Bump time dependency

## [0.18.23] - 2024-07-25

### Bug Fixes

- Disable broken freebsd tests

### Documentation

- Clear up confusion around ls

### Miscellaneous Tasks

- Release eza v0.18.23

### Build

- Bump log from 0.4.21 to 0.4.22
- Bump DeterminateSystems/nix-installer-action from 12 to 13
- Bump plist from 1.6.1 to 1.7.0

## [0.18.22] - 2024-07-18

### Bug Fixes

- Use NaiveDateTime::from_timestamp_opt instead of panicky From impl

### Features

- Add non-nix pre-commit rustfmt and clippy hooks

### Miscellaneous Tasks

- Release eza v0.18.22

### Ci

- Bump FreeBSD version.

## [0.18.21] - 2024-07-01

### Bug Fixes

- Fix missing line breaks in _eza

### Miscellaneous Tasks

- Release eza v0.18.21

## [0.18.20] - 2024-06-27

### Features

- Add --no-|show-symlinks flags for filtering output

### Miscellaneous Tasks

- Release eza v0.18.20

## [0.18.19] - 2024-06-20

### Bug Fixes

- Ship release binaries

### Miscellaneous Tasks

- Release eza v0.18.19

### Build

- Bump git2 from 0.18.3 to 0.19.0

## [0.18.18] - 2024-06-13

### Features

- Extend deny check in audit workflow to all
- Add deny.toml and workflow file to audit workflow paths
- Run on all features by default
- Ask for shell and terminal in bug report template

### Miscellaneous Tasks

- Release eza v0.18.18

### Build

- Bump unicode-width from 0.1.12 to 0.1.13
- Bump DeterminateSystems/flake-checker-action from 7 to 8
- Bump DeterminateSystems/nix-installer-action from 11 to 12

## [0.18.17] - 2024-06-05

### Features

- Add icon for Nushell extension

### Miscellaneous Tasks

- Release eza v0.18.17

### Build

- Bump trycmd from 0.15.1 to 0.15.2
- Bump libc from 0.2.154 to 0.2.155

## [0.18.16] - 2024-05-16

### Bug Fixes

- Change windows-only imports to be windows-only

### Documentation

- Replace decay with color-scale
- Update INSTALL.md
- Fix typo in `INSTALL.md`
- Use 3 columns for packaging status badge

### Miscellaneous Tasks

- Release eza v0.18.16

### Build

- Bump DeterminateSystems/flake-checker-action from 5 to 7
- Bump DeterminateSystems/nix-installer-action from 10 to 11

## [0.18.15] - 2024-05-09

### Bug Fixes

- Correct command for latest tag in deb-package.sh

### Documentation

- Add how to find man pages in terminal and online. Partly fixes #967
- Correct heading levels in markdown
- Move heading out of collapsed section
- Add some keywords for benefit of ctrl-f

### Features

- Return to original commit at the end of deb-package.sh
- Add optional tag argument to deb-package.sh

### Miscellaneous Tasks

- Release eza v0.18.15

## [0.18.14] - 2024-05-02

### Features

- Add icon for "cron.minutely" directory

### Miscellaneous Tasks

- Release eza v0.18.14

### Build

- Bump uzers from 0.11.3 to 0.12.0
- Bump libc from 0.2.153 to 0.2.154
- Bump unicode-width from 0.1.11 to 0.1.12
- Bump palette from 0.7.5 to 0.7.6

## [0.18.13] - 2024-04-25

### Bug Fixes

- Allow unused imports for freebsd
- Checking for deref flag in file_name

### Features

- Add scheme filetype and icons
- Generate completion/manpage tarballs on release

### Miscellaneous Tasks

- Release eza v0.18.13

## [0.18.11] - 2024-04-19

### Bug Fixes

- Fix clippy lints
- Enable the rule only for NetBSD.
- Build aarch64, arm without libgit2

### Miscellaneous Tasks

- Release eza v0.18.11

### Ci

- Bump NetBSD version to 10.0

## [0.18.10] - 2024-04-11

### Bug Fixes

- Bump trycmd from 0.15.0 to 0.15.1

### Miscellaneous Tasks

- Release eza v0.18.10

### Build

- Bump nu-ansi-term from 0.49.0 to 0.50.0

## [0.18.9] - 2024-03-27

### Features

- Switch out ansiterm crate for nu_ansi_term

### Miscellaneous Tasks

- Release eza v0.18.9

### Build

- Bump DeterminateSystems/nix-installer-action from 9 to 10
- Bump plist from 1.6.0 to 1.6.1
- Bump rayon from 1.9.0 to 1.10.0
- Bump git2 from 0.18.2 to 0.18.3

## [0.18.8] - 2024-03-21

### Bug Fixes

- Avoid deprecation warnings
- Rustfmt issues

### Features

- Add fennel lang icon and associations

### Miscellaneous Tasks

- Release eza v0.18.8

## [0.18.7] - 2024-03-14

### Bug Fixes

- Bugfix to resolve absolute paths that are not symlinks

### Features

- Add filetype and icon for .hh extension

### Miscellaneous Tasks

- Release eza v0.18.7

## [0.18.6] - 2024-03-06

### Bug Fixes

- NetBSD did not have fflagstostr and as such did not build properly
- Fix total-size option
- Add fortran to source filetypes
- Fix absolute_path() for broken symlinks
- Update line numbers in panic messages in tests

### Features

- Add filetype and icon for age
- Adding icons for graphql extensions
- Add nim icons
- Use fsharp icon for fsproj files (similar to cs/csproj)
- Add new icons, diverse selection
- Adding more haskell related icons
- Adding more icons for docker specific files
- Adding more dockerfile icons
- Add --absolute flag
- Add shell completions for --absolute flag

### Miscellaneous Tasks

- Cleaning dirs
- Release eza v0.18.6

### Refactor

- Port grid and grid-details to new uutils-term-grid

### Testing

- Add integration tests and powertests for --absolute flag
- Add directory symlink to tests/itest/

### Build

- Bump log from 0.4.20 to 0.4.21
- Bump rayon from 1.8.1 to 1.9.0

### Ci

- Add NetBSD to CI.
- Fix warnings.
- Add FreeBSD to CI.
- Add OpenBSD to CI.

## [0.18.5] - 2024-02-29

### Bug Fixes

- Bump palette from 0.7.4 to 0.7.5

### Miscellaneous Tasks

- Release eza v0.18.5

## [0.18.4] - 2024-02-22

### Bug Fixes

- Classification width should be taken into account with -F=auto

### Miscellaneous Tasks

- Release eza v0.18.4

### Build

- Bump libc from 0.2.152 to 0.2.153
- Bump chrono from 0.4.33 to 0.4.34
- Bump trycmd from 0.14.20 to 0.15.0

## [0.18.3] - 2024-02-15

### Bug Fixes

- Duplicates in shell completions

### Documentation

- Add target arch to deb PPA installation for strict apt environments

### Miscellaneous Tasks

- Release eza v0.18.3

### Performance

- Do not pre-compute MountInfo to reduce readlink calls

### Refactor

- Use #[default] attribute instead of custom impl for enums

## [0.18.2] - 2024-02-08

### Bug Fixes

- Update libgit2 to 1.7.2

### Miscellaneous Tasks

- Release eza v0.18.2

## [0.18.1] - 2024-02-08

### Bug Fixes

- Change shasum for main commit

### Documentation

- Add manual installation section

### Miscellaneous Tasks

- Release eza v0.18.1

### Refactor

- Replace scoped_threadpool with rayon

### Build

- Add empty rustfmt to ensure project specific settings
- Bump libc from 0.2.151 to 0.2.152
- Bump nick-fields/retry from 2 to 3
- Bump palette from 0.7.3 to 0.7.4
- Bump webiny/action-conventional-commits from 1.2.0 to 1.3.0

## [0.18.0] - 2024-02-01

### Features

- [**breaking**] Add --classify=always,auto,never

### Miscellaneous Tasks

- Remove rustfmt config file that has a nightly only option in favor of rustfmt skip directive which is already in place
- Fix small typo in pull request template
- Release eza v0.18.0

### Refactor

- Change cast to coertion, remove rustfmt skip and clippy lint ignore directives

### Testing

- Regenerate classification related tests

### Build

- Change flake inputs

## [0.17.3] - 2024-01-25

### Bug Fixes

- Remove version testing

### Miscellaneous Tasks

- Avoid `unwrap()` by changing filter-then-map to `filter_map`
- Release eza v0.17.3

### Build

- Bump shlex from 1.2.0 to 1.3.0
- Bump chrono from 0.4.31 to 0.4.33
- Bump trycmd from 0.14.19 to 0.14.20

## [0.17.2] - 2024-01-20

### Bug Fixes

- Crash using --git-repos on unreadable dir
- Crash using --git-repos on unreadable dir

### Miscellaneous Tasks

- Release eza v0.17.2

### Build

- Add cargo-bump for releasing

## [0.17.1] - 2024-01-11

### Bug Fixes

- Offset widths in grid mode with utf8 filenames
- Format the code
- Unformat the code where needed
- Format the code correctly this time
- Redo everything from scratch
- Stack overflow when '-laaR' are used
- Stack overflow when '-laaR' is used

### Features

- Add Fortran icons

### Miscellaneous Tasks

- Adding blake3 to checksums
- Release eza v0.17.1

### Testing

- Regenerate version tests... and others
- Updated tests to fit new features

### Build

- Add b3sum to devshell deps

## [0.17.0] - 2023-12-13

### Bug Fixes

- Add color scale mode to the bash completions
- Add color scale mode to the fish completions
- Quote symbolic links correctly when their destinations contain spaces

### Documentation

- Modify documentation about custom time style

### Features

- Add BSD file flags
- Add Windows file attributes
- [**breaking**] Support different custom time style for non-recent/recent files

### Miscellaneous Tasks

- Release eza v0.17.0

### Testing

- Regen powertests
- Regenerate
- Add tests for non-recent/recent custom time style
- Update powertest expected help message output

### Build

- Update `flake.lock`
- Bump DeterminateSystems/nix-installer-action from 8 to 9
- Bump once_cell from 1.18.0 to 1.19.0
- Bump libc from 0.2.150 to 0.2.151

### Ci

- Remove labeler

## [0.16.3] - 2023-12-07

### Bug Fixes

- Add bare git_repos fn if feature git is disabled
- Fixing color of size unit
- Color-scale broked size for colors

### Miscellaneous Tasks

- Release eza v0.16.3

### Testing

- Fix powertests post-release

### Build

- Bump percent-encoding from 2.3.0 to 2.3.1
- Bump actions/labeler from 4 to 5

## [0.16.2] - 2023-11-30

### Bug Fixes

- Calculate width correctly when using grid icons & classify
- Fix the windows build

### Miscellaneous Tasks

- Release eza v0.16.2

### Testing

- Fix version tests

### Build

- Bump webiny/action-conventional-commits from 1.1.0 to 1.2.0
- Bump DeterminateSystems/nix-installer-action from 7 to 8
- Bump windows-sys from 0.48.0 to 0.52.0

## [0.16.1] - 2023-11-23

### Bug Fixes

- Don't panic with todo!() on inaccessible dir
- Don't panic if the btime of a file is Err
- Lifetime annotations and manpage/shell completion nits
- Reflow help

### Features

- Handle formatting and display of binary extended attributes.
- Add netbsd and freebsd support for extended attributes

### Miscellaneous Tasks

- Update flake inputs
- Release eza v0.16.1

### Testing

- Vars mocking
- Display and meta options
- Filtering and sorting
- Long view options
- Regenerate `--help` tests

### Build

- Sign release tags

## [0.16.0] - 2023-11-16

### Bug Fixes

- Fix cross compilation
- Snap requires a base
- Move `--smart-group` to long view options
- Colo[u]r-scale didn't have a base value
- Fix snapcraft.yaml formatting

### Documentation

- Add comments for bzip variants
- Added the fact that total-size is unix only

### Features

- Add some file extensions
- Abort on panic (saving 0.1 M)
- Add powertest

### Miscellaneous Tasks

- Ignore blame from #644
- Stabilize powertest version
- Release eza v0.16.0

### Testing

- Implements tests using the generated directory
- Powertests using generated testdirs
- Add hashed versions of powertests

## [0.15.3] - 2023-11-09

### Bug Fixes

- Changed quote in --almost-all completion
- [**breaking**] Remove Repo column when using --git-repos when no git repo
- Reformat `help.rs`
- Allow unused macro rule arms

### Documentation

- Improve CONTRIBUTING.md, README.md
- Improve README.md
- Introduce INSTALL.md

### Features

- Create EZA_ICONS_AUTO environment variable
- Create EZA_ICONS_AUTO environment variable
- Demo gif and gif generation recipe
- Add ocaml icon filetypes
- Add PRQL
- Add `--color-scale`

### Miscellaneous Tasks

- Add to CODEOWNERS file to make sure I get ping'd on files being touched
- Add myself to codeowners to watch modifications on parsing
- Improve the PR template
- Release eza v0.15.3

### Refactor

- Remove commented out test code
- Finalize `decay` -> `color_scale`

### Build

- Refactor flake
- Bump libc from 0.2.149 to 0.2.150
- Bump DeterminateSystems/nix-installer-action from 4 to 7
- Bump rustix from 0.38.13 to 0.38.21

### Ci

- Refactor pre-commit-hooks
- Refactor publish workflow

## [0.15.2] - 2023-11-02

### Bug Fixes

- Correct width when --no-quotes is used
- Clippy lint and add option to grid-details
- --smart-group only works for current user

### Features

- Add Typst to the recognized files

### Miscellaneous Tasks

- Release eza v0.15.2

### Refactor

- Replace `lazy_static` with `once_cell`
- Replace plain values with TextColours

### Testing

- Added more content to the dir generator
- Changed size of one of the files

## [0.15.1] - 2023-10-26

### Bug Fixes

- Only store top-level recursive dir size
- Changed windows methods
- Underscored unused windows variables
- Added device for filesystem to hashmap
- Don’t display target’s size if we’re not dereferencing
- Display offset for filenames with spaces
- Fix clippy warnings
- Fix doc-tests on RecursiveSize
- Fix dead_code warnings on Windows

### Documentation

- Fix doc-tests formatting and address other documentation review requests

### Features

- Add a new filetype for source code files
- Add a new icons for source code files and other files
- Support for displaying blocksize on directories

### Miscellaneous Tasks

- Release eza v0.15.1

### Refactor

- Move total-size calculations to File
- Add RecursiveSize type to simplify total-size calculation

## [0.15.0] - 2023-10-19

### Bug Fixes

- Reenable debug symbols in debug builds
- Fmt, windows, and nix fixes
- Reverted autofmt changes
- Updated match indents
- Changed flag name
- Clippy lint
- Merge conflict with main

### Documentation

- Correct color option spellings
- Added flag to readme
- Added flag to man

### Features

- Add option --smart-group
- Add completions, man for --smart-group
- Added recursive directory parser
- Added flag to completions
- Add icons=always,auto,never. dont display icons in a tty|piped
- Fix auto value for colors and icons + documentation
- [**breaking**] Remove --no-icons in favor of --icons=always,auto,never. default is auto

### Miscellaneous Tasks

- Upgrade to uutils_term_grid from unmaintained term_grid
- Release eza v0.15.0

### Build

- Bump DeterminateSystems/nix-installer-action from 5 to 6

### Ci

- Remove stalebot, is super annoying
- Adjust test case to icons=auto (no icons should show due to tty)

## [0.14.2] - 2023-10-12

### Bug Fixes

- Comment out redundant static musl build
- Refactor sed command to build manpages
- Update additional completions for help, almost-all, dereference
- Fix zsh completions

### Documentation

- Add missing options to man page and CLI --help info

### Features

- Add missing nu shell completions
- Adding the EZA_OVERRIDE_GIT env var

### Miscellaneous Tasks

- Release eza v0.14.2

### Refactor

- Use musl target for amd64 deb package
- Directly use one "big" awk command

### Styling

- Remove trailing spaces and trailing line

### Build

- Bump libc from 0.2.148 to 0.2.149
- Bump DeterminateSystems/nix-installer-action from 4 to 5

## [0.14.1] - 2023-10-08

### Bug Fixes

- Replace left-over exa in fish completion
- Diabling static linked binaries due to segfault
- Make os error 13 fail loud
- Root group not painted as expected when eza used by root
- Adjust change width calculations for hyperlink and classify

### Documentation

- Correct CONTRIBUTING.md on commit message type
- Fix typos
- Add zsh with homebrew part to completions section
- Installation on fedora updated

### Features

- Add basic nushell completion file
- Add codeowner for nu completions
- Readded musl static bin as it works

### Miscellaneous Tasks

- Release eza v0.14.1

### Refactor

- Align completions
- Do not match for numbers and remove single-use fn
- Consistent argument order

### Testing

- Classify-hyperlink test case for width 50
- Move classify tests to local

### Build

- Bump trycmd from 0.14.17 to 0.14.19
- Make checksums easier to copy-paste
- Improve release automation
- Fix version bump
- Fix double echo
- Automate gh release
- Add `codegen-units = 1` and remove `opt-level = 3`
- Add back `opt-level = 3`

### Ci

- Treat warnings as errors

## [0.14.0] - 2023-10-02

### Bug Fixes

- Ignore refs for blame
- Avoid unstable inner attributes
- Merge conflict with main
- Merge conflict with main
- Fix manpage generation of default package
- Changed dll icon
- Changed readme and Added README icon
- New R lang icon
- README is sorted and formatted
- Fix large_enum_variant warning with explanation
- Query stdout terminal size to see if the output gose to a tty.
- Use windows-specific API for terminal size query on windows
- Add `windows-sys` dependency for targeting windows
- Use `std::io::IsTerminal` to eliminate compatibility issue
- Terminal size query should only check `stdout`
- Prefix unused binding name with underscore

### Documentation

- Add completions + manpage for --no-quotes flag
- Leave nix install instructions open-ended
- Adding termux section
- Leave nix install instructions open-ended
- Added the new colors option to the man
- Documenting custom time-style
- Time-format supporting custom formats
- Updated man to add new colors
- Description of `--color` in README, manpage, and completions
- Change `color` to `colo[u]r` in the option description.

### Features

- Add rustfmt.toml file to prevent flags.rs fmt on save
- Add quotations around filenames with spaces. exa pr#1165
- Replace hardcoded version by version variable
- Add header to colors-explanation page
- Revise man rule to use for loop and insert version
- Adding the possibility to change git-repos colors
- [**breaking**] Separated root from other users
- New Rust icon
- Added bdf,psf icons
- Added lib icon
- Added Contacts,Favorites icons
- Added home icon
- Added fdmdownload icon
- Added statically linked binaries

### Miscellaneous Tasks

- Release 0.14.0

### Refactor

- Ignore options/flags.rs
- Renamed and reintended some code
- Reformatted a line

### Styling

- Format some parts correctly

### Build

- Bump unicode-width from 0.1.10 to 0.1.11
- Bump git2 from 0.18.0 to 0.18.1
- Temporarily disable aarch64-unknown-linux-gnu
- Name static binaries

## [0.13.1] - 2023-09-25

### Bug Fixes

- Typo `this` -> `that`
- Don’t show color when color is disabled
- Respect spec on Windows and make it for with Konsole
- Major and minor device on MacOS
- Linux uses u32 for major/minor device numbers
- Error for missed semicolon
- More than 3 bools in a struct
- Enable rustfmt by removing .rustfmt.toml which disables it
- Replace rustfmt::skip on expressions because experimental
- Remove unnecessary rustfmt::skip's in windows code
- Add src/options/flags.rs to rustfmt.excludes
- Left-over merge conflict in src/output/table

### Documentation

- Update README.md
- Update --mounts option to include MacOS
- Documenting --only-files

### Features

- Add EXA_COLOR bindings for un-themed items
- Add EZA_ environment variables with fallback to EXA_
- Listing files only using '--only-files' flag
- Add rustfmt check to unit-tests workflow

### Miscellaneous Tasks

- Add completion for --only-fies (zsh,fish)
- Release 0.13.1

### Refactor

- Fix rustfmt issues and place skips where needed
- Reorder unit-tests to fmt, clippy and tests

### Styling

- Formatted using treefmt
- Fix clippy warning after rustfmt
- Fix treefmt issues in options module
- Reapply rustfmt after rebase from main

### Testing

- Add unit tests for new style abbreviations
- Regen git_repos_no_status
- Test for listing files only

### Build

- Bump actions/checkout from 2 to 4
- Bump chrono from 0.4.30 to 0.4.31
- Bump timeago from 0.4.1 to 0.4.2
- Bump libc from 0.2.147 to 0.2.148
- Bump terminal_size from 0.2.6 to 0.3.0

### Ci

- Added formatters to treefmt
- Make various improvements
- Only apply labels when opening a PR

## [0.13.0] - 2023-09-18

### Bug Fixes

- Crate can't contain broken symlink
- Remove executable flag from fish completion file
- Use proc_mounts only on linux
- Hotfix harmful documentation
- Fix hyperlinks on Windows
- Needless_borrow
- Nix flake check also builds the package
- [**breaking**] Change number_huge and unit_huge to match the man page short codes

### Documentation

- Added instructions to install completions of eza to the readme
- Added cafkafk suggestions
- Fix codeblocks in zsh completions
- Update README.md
- Add Winget install info
- Link directly to space
- Document new file type two letter codes in man page
- Document filetypes theme and rename trait
- Update deb instructions to use keyring
- Fix chmod in deb installation instructions
- Add potential gpg install to deb installation instructions
- Add install instructions for Void Linux
- Document dimmed and italic style codes
- Document character style pairs in the code and match with man page
- Documentation of 'sn' and 'sb' conflicted with later docs

### Features

- Add completion files in deb packaging script
- Adds filtering for Windows hidden files
- Make file types themeable
- Lazy loading of a files extended attributes and absolute path

### Miscellaneous Tasks

- Augment gitter size in README
- Release 0.13.0

### Performance

- Add criterion for benchmarking

### Refactor

- Refactor just in crossfile
- DRY up justfile
- Ignore missing MSVC docker image
- Removed unused imports, mark mods as allow unused
- Format code
- Move ALL_MOUNTS to fs::mounts
- Migrate ALL_MOUNTS from lazy_static to OnceLock
- Rename FileType::Immediate to more obvious FileType::Build

### Testing

- Autogenerate testing dir
- Stabalised unit-tests.yml
- Autogenerate test dirs
- Generate device files
- Add unit tests that test both exa and ls style codes together
- Address variable names

### Build

- Set optlevel to 3
- Add musl binary for linux
- Fix checksums
- Add TODOs to targets

### Ci

- Add Winget Releaser workflow
- Add nix Flake check to flake.yml
- Removed nix build in favor of nix flake check
- Include bash completion script in treefmt and fixed shellcheck formatting in completion script
- Fix spelling attemps -> attempts

## [0.12.0] - 2023-09-14

### Bug Fixes

- RUSTSEC-2020-0071
- Expand `--all` help
- Generalize gitignore to ignore all eza deb packages
- Change trycmd config to use test/itest folder for testing
- Revert to old apt install command suggestion and add hint
- Canonicalize errors when the destination of a symbolic link is bad
- Handle other canonicalize errors in hyperlinks and git
- Fix windows build when canonicalize returns an error
- Is_some_and is an unstable Rust feature until 1.70
- Remove stray backslashes
- Exit 13 on os error 13
- Rewrite comment
- Improve trace strings
- Tracing typo
- Revert "Support for Windows Hidden Files"
- Shellcheck warnings
- Revert "Support for Windows Hidden Files"
- Shellcheck warnings

### Documentation

- Expand `--all` documentation
- Add gentoo
- Fix gentoo install
- Add MacPorts install info
- Add pthorpe92 gist
- Add docs for --git-repos & --git-repos-no-status
- Fix gpg armor flag for deb release in readme
- Add scoop install info
- Add Mac support for the --mount option in the man page
- Add SAFETY comments to unsafe code blocks
- Remove license from developemnt section
- Update rust badge
- Add better explanation of git repos + no status
- Remove color specifications. change unknown git repo status to `~`
- Fix missing color specification from man page
- Add missing man page for debian release

### Features

- Add audit workflow
- Add trycmd as dev-dependency
- Add minimal trycmd binary
- Add a few trycmd tests as example
- Add apt installation workflow
- Support --mount option on Mac
- Support --mount option on Mac
- Adds filtering on Windows hidden files
- Document and change output for --git-repos
- Add PERMISSION_DENIED exit code
- Adds filtering on Windows hidden files
- Adds filtering on Windows hidden files
- Added shellcheck to treefmt
- Adds filtering on Windows hidden files

### Miscellaneous Tasks

- Bump uzers to v0.11.3
- Bump chrono from 0.4.27 to 0.4.30
- Removal of xtests
- Removal of vagrant
- Remove deprecated devtools
- [**breaking**] MSRV 1.70
- Run spellcheck
- Release 0.12.0

### Refactor

- Over-engineer deb-package.sh
- Hide xtests folder
- Split trycmd into tests for all, unix and windows
- Limit unit-tests run on workflow change to unit-tests itself
- Add tracing to various code parts
- Make std::process::exit global
- Moved generateTest.sh to devtools/
- Renamed the file

### Revert

- "Support for Windows Hidden Files"

### Styling

- Remove TODO message on the absolute_path property
- Fix shellcheck issues in deb-package.sh
- Fix shellcheck issues in deb-package.sh
- Fix shellcheck issues in deb-package.sh

### Testing

- Remove vhs from flake
- Remove vhs-runner files
- Dump trycmd from nix sandbox
- Fix name of trydump
- Add trycmd
- Add nix feature
- Add example long tests for sandbox
- Set itests files to unix epoch
- Set itest files to unix epoch
- Refactor setting unix epoch
- Auto discard old definitions
- Fix test reference
- Add long_all_nix.toml
- Add long_blocksize_nix.toml
- Add long_extended_nix.toml
- Add long_git_nix.toml
- Add long_git_repos_nix.toml
- Add long_git_repos_no_status_nix.toml
- Add long_grid_nix.toml
- Add long_header_nix.toml
- Add long_icons_nix.toml
- Add long_octal_nix.toml
- Add long_time_style_relative_nix.toml
- Freeze nix tests
- Fix trydump when no files to delete
- Adding more content to test
- Modified unix and all tests
- Regenerate nix tests
- Convert windows tests with new itest dir
- Fixed windows tests being wrong
- Added a test generator
- Add more unix_tests
- Fixed unix tests to remove any distro specific
- Removed git test breaking on nix
- Remove non-deterministic test

### Build

- Add compression, checksum gen for bin
- Add deny.toml
- Update flake.lock, cargo.lock
- Remove org warnings
- Remove itest
- Update flake.lock
- Add itest, idump
- Make trycmd part of checks

### Ci

- Don't use nix feature on ci
- Fix windows build
- 1.65 -> 1.70
- Enforce conventional commits
- Enforce conventional commits

### Doc

- Remove xtests section from readme
- Add deprecation warning to xtests/readme
- Add deprecation warning to just xtest commands
- Add deprecation warning to vagrantfile
- Add guidelines for commit messages

## [0.11.1] - 2023-09-11

### Bug Fixes

- Add vendored-libgit2 feature to git2 dependency
- Filename escaping (last character lost sometimes, no hyperlink)
- Build for Windows with chrono

### Documentation

- Explain vendored-libgit2
- Add homebrew, misc fixes
- Fix code of conduct link
- Update archlinux
- Remove broken dependabot link
- Add informaton about lazy_static
- Add star history
- Add bright color options in man pages
- Add bright color support in readme changelog

### Features

- Add highlighting of mounted directories (Linux only)
- Mark `.git` as ignored, which hides it when using `--git-ignore`
- Expose git2 feature vendored-libgit2
- Add build commands to deb-package.sh
- Add bright colour options, change punctuation default
- Support the MSRV of Rust (1.65.0)
- Use chrono crate to handle datetime-related features

### Miscellaneous Tasks

- Bump actions/checkout from 3 to 4
- Release 0.11.1

### Testing

- Stabilize testing without sandbox
- Disable gif rendering

### Build

- Add release binaries
- Fix binary gen
- Add armhf binary

### Deps

- Change ansi_term to ansiterm from rustadopt

## [0.11.0] - 2023-09-04

### Bug Fixes

- Add windows implementation of is_empty_dir
- Re-align `--git-ignore` in help message
- Avoid direnv error if nix isn't installed

### Documentation

- Empty dir functions
- Document is_empty_dir functions
- Add function documentation for get_file_type and icon_for_file.

### Features

- Optimize checking for empty directories when a directory has subdirectories
- Use perfect hash tables for file types and icons
- Add backlog of icons from various exa pull requests and others
- Add backlog of icons from various exa issues

### Miscellaneous Tasks

- Bump git2 from 0.17.2 to 0.18.0
- Bump uzers from 0.11.1 to 0.11.2
- Bump DeterminateSystems/flake-checker-action from 4 to 5
- Bump DeterminateSystems/nix-installer-action from 3 to 4
- Bump glob from 0.3.0 to 0.3.1
- Bump actions/stale from 5 to 8
- Bump terminal_size from 0.1.16 to 0.2.6
- Bump timeago from 0.3.1 to 0.4.1
- Release 0.11.0

### Refactor

- Use phf macros instead of codegen to create icon and filetype tables
- Add constants for most of the commonly used icons
- Add constants for the rest of icons used multiple times
- Rename class FileExtension to FileTypeClassifier to better reflect the purpose
- Move get_file_type to FileType enum

### Styling

- Is_empty_dir() was put between the unix size() and windows size()

### Build

- Use rust stable
- Add unstable package
- Disable clippy check 'unreadable_literal' in generated files

## [0.10.9] - 2023-08-28

### Bug Fixes

- Respect git-repos flags

### Documentation

- Add badge for eza gitter/matrix room
- Fix matrix link
- Add ignored flags to readme
- Add ignored flags to manual
- Add ignored flags to help
- Add ignored flags to xtest

### Features

- `--no-git` option

### Miscellaneous Tasks

- Add funding.yml
- Release 0.10.9

### Tree-wide

- Fix Windows build

### Build

- Add convco to dev

### Ci

- Create flakehub-publish-tagged.yml
- Add workflow_dispatch to flakehub-pub
- Edit workflow_dispath
- Refactor workflow_dispath
- Refactor workflow_dispath
- Remove broken dispatch
- Add flakehub-backfill
- Add codeowners
- Add gierens as .deb codeowner
- Add windows to CI

## [0.10.8] - 2023-08-22

### Bug Fixes

- TextCell building of detailed grid view for  hyperlink and icon options
- Block's Colours trait as for file sizes
- --blocksize completion, new description
- Option.views unit tests use --blocksize
- Add missing colon before -w/--width
- Replace exa by eza in help string
- Change exa to eza in invalid option error
- Add missing name section to eza_colors-explanation manpage
- Replace exa by eza in .gitignore

### Documentation

- Update issue templates
- Cafkafk -> eza-community

### Features

- Add git-ignored color/style option
- Add `just` and `pandoc` to devShell bc they are necessary for man
- Add `.envrc` so direnv automatically opens the nix dev environment
- Match folder icon to reflect contents
- Match folder icon to reflect contents
- --blocksize completion, new description
- Add script deb-package.sh

### Miscellaneous Tasks

- Bump git2 from 0.16.1 to 0.17.2
- Bump unicode-width from 0.1.8 to 0.1.10
- Bump libc from 0.2.93 to 0.2.147
- Bump num_cpus from 1.13.0 to 1.16.0
- Release 0.10.8

### Refactor

- Fs::fields::Blocks
- File::blocks() name, revise calculation
- Rendering Blocksize like file sizes
- Rename Blocks column to Blocksize
- Use -S/--blocksize and, var BLOCKSIZE
- Unit tests for output.render.blocks
- Flip if (as suggested/demanded by clippy)
- Migrate to uzers lib

### Build

- Add charm to nix develop
- Add tests/tmp to gitignore
- Add initial tape
- Add test runner sketch
- Add test runner to justfile
- Add out.gif to .gitignore
- Add run_tests  NAME arg
- Add reference main.txt
- Add gen_test
- Fix typo
- Handle arbitrary NAMES
- Remove commented out code
- Fix code formatting
- Add vhs-runner main function
- Gen_test support automatic gen
- Automatic tape detection
- Add print_msg with ansi color
- Slight documentation/refactor
- Use ansi output on all output
- Disable vhs publish ad
- Add better tracing
- Remove defective sed
- Add color variables
- Add eza-long test
- Add itest testing dir
- Add parallel runner

### Ci

- Help text in xtests
- Nix flake check
- Add labeler for flake
- Add flake description

### Deps

- Change users depedency to uzers

### Doc

- Add git-ignore style/color information to manpage
- --blocksize, new description
- --blocksize, new description
- --blocksize, new description
- Add gpg public key for the deb repository
- Add section about debian and ubuntu installation

### Git

- Add deb package to .gitignore

## [0.10.7] - 2023-08-13

### Bug Fixes

- Respect GIT_CEILING_DIRECTORIES
- MacOS flake support
- Broken zsh completion syntax

### Features

- Add gitlab-ci.yml
- Improve icon for Earthfile
- Better.ps1, add .psd1, .psm1 icons
- Replace .bat icon by windows cli icon
- Use TeX icons and add .bib, .bst icon
- Use Ocaml logo, add .mli, .mll, .mly
- Add many more icons
- Add -w/--width to help string
- Add -w/--width to README
- Add -w/--width to flags
- Add -w/--width to manpage
- Fish -w/--width
- Zsh -w/--width

### Miscellaneous Tasks

- Add PR template
- Bump log from 0.4.14 to 0.4.20
- Release 0.10.7

### Refactor

- GIT_DIR handling
- Turn unused var into value
- Fix borrowed trait implements required
- Simplify format strings
- Consistent style
- Clippy::explicit_auto_deref
- Clippy::explicit_auto_deref
- Clippy::redundant_else
- Clippy::manual_map
- Clippy::semicolon_if_nothing_returned
- Clippy::extra_unused_lifetimes
- Allow clippy::wildcard_in_or_patterns
- Clippy::uninlined_format_args
- Allow Colours::new call with self
- Clippy::explicit_iter_loop
- Clippy::uninlined_format_args
- Clippy::needless_late_init
- Clippy::useless_conversion
- Clippy::implicit_clone
- Clippy::uninlined_format_args
- Clippy::into-iter-on-ref
- Clippy::semicolon_if_nothing_returned
- Clippy::into_iter_on_ref
- Clippy::needless_lifetimes
- Clippy::uninlined_format_args
- Trivial clippy lints
- Clippy::semicolon_if_nothing_returned
- Clippy::semicolon_if_nothing_returned
- Clippy::manual_let_else
- Clippy::semicolon_if_nothing_returned
- Clippy::semicolon_if_nothing_returned
- Clippy::uninlined_format_args
- Clippy::manual_let_else
- Clippy::manual_let_else
- Clippy::manual_let_else
- Clippy::manual_let_else
- Clippy::manual_let_else
- Fix trivial cast
- Clippy::needless-borrow
- TerminalWidth::deduce to -w/--width

### Ci

- Create pull_request_template.md
- Add clippy check
- Add dependabot updater

### Doc

- Create SECURITY.md
- Create CONTRIBUTING.md

## [0.10.6] - 2023-08-07

### Bug Fixes

- Rename eza-colors-explanation
- Exa -> eza in manpage

### Documentation

- Adding --git-repos to help.

### Features

- Use GIT_DIR env var to find the repo
- Add color explanations

### Miscellaneous Tasks

- Release 0.10.6

### Doc

- Add aur, nixpkgs installation

### Git

- Use GIT_DIR env var to find the repo
- Use open_from_env before discover

## [0.10.5] - 2023-08-03

### Bug Fixes

- Output wraps in terminal
- Respect icon spacing

### Miscellaneous Tasks

- Release 0.10.5

## [0.10.4] - 2023-08-02

### Bug Fixes

- Syntax error

### Features

- Added ".out" files for latex
- Add changelog generation

### Miscellaneous Tasks

- Release 0.10.4

## [0.10.3] - 2023-07-31

### Bug Fixes

- More JPG extensions
- Add compression icon to .tXX files #930
- Dereferencing linksfile size.
- Dereferencing links users.
- Dereferencing links groups.
- Dereferencing links permissions.
- Dereferencing links timestamps.
- Add Svelte icon
- Fish completion for -i/--inode option
- Typo
- Use eprintln instead
- Use stderr on no timezone info
- Bump openssl-src from 111.15.0+1.1.1k to 111.26.0+1.1.1u
- Bump openssl-src from 111.15.0+1.1.1k to 111.26.0+1.1.1u
- Changed bin name via cargo.toml
- Change man pages to reffer to new binary name
- Change completions to new binary name
- Change completion file names
- Change name to eza
- Bump git2 from 0.13.20 to 0.16.1
- Fixed grid bug
- Fixed grid bug
- Bump rust to 1.71.0
- Take -a and -A equally serious
- Changed default folder icon
- Add clippy as part of the toolchain
- Change license icon
- Change gpg icons to keys
- Add icon for ocaml (.ml extension)
- Better license icon
- Replace obsolete icons
- Add Emacs icon for .el and org-mode for .org
- Added icons for .rmeta
- Add icon support for .mjs, .cjs, .mts, .cts files
- Add webpack.config.cjs to immediate files list
- .ipynb icon comment
- Removed result
- Update --version info
- Sort is_immediate
- Add flake, autoconf, cargo lock
- Added trailing commas
- Update snapscraft.yaml
- Remove accidentally commited test files

### Feat

- Add JPF to image filetype

### Features

- Add support Typescript and ReasonML projects
- New Icons and CLI argument to suppress icons
- Add sty file
- Add julia file extension icon
- Add symlink dereferencing flag
- Add -X/--dereference completions
- Add -X/--dereference completions
- Symlinks report their target's valid size
- Update Cargo.toml to optimise binaries for size
- Add status for git repos
- Add selinux contexts support
- Add -o shorcut to --octal-permissions
- Hyperlink flag
- Update Cargo.toml to optimise binaries for size 
- Add git-status-.* completions
- Zsh add git-status-.* completions
- Add git-status-.* completions
- Add Zig module icons
- Add icon for Vagrantfile
- Add git icon to .gitignore_global file
- Left align relative time
- Add support for --time-style=relative
- Add vim icon
- Add justfile
- Add pxm
- Add compressed types
- Add compressed icons

### Fixup

- Split prefix tests by property

### Improve

- Vim icon

### Makefile

- Be compatible with BSD and OS X

### Miscellaneous Tasks

- Update zoneinfo_compiled, datetime to 0.5
- Update users to 0.10
- PR feedback
- Bump to v0.10.2
- Bump to v0.10.3
- Update cargo lock

### Refactor

- Use shorthand fields
- Removed commented code
- Sorted file types, color table

### StatResult

- :Path -> Dir

### Styling

- Add icon for reStructuredText (src) files

### Testing

- Change to /usr/bin/env bash

### ToStr

- :to_str -> ToString::to_string

### Add

- Mp2 audio format icon

### Build

- Use binary name only

### Ci

- Remove unused .github files
- Remove unused .github files
- Create unit-tests.yml
- Create unit-tests.yml
- Add trivial nix flake
- Add treefmt, rust-toolchain, nixfmt
- Add .#test, .#clippy, .#check
- Add nix flake
- Change branch
- Bump rust to 1.71.0
- Automatically mark issues/PRs stale
- Run tests when building with nix
- Moving actions to dtolnay's version
- Update Cargo.toml
- Create labeler.yml
- Add snap to labeler.yml
- Add filetype.rs autolabel

### Details

- `filter` is only used when recursing

### Doc

- Add -X/--dereference flag
- Change name in README.md
- Add `nix run` to readme
- Fix flow issue
- Fix typos
- Add mandatory snowflake emoji
- Document nix flake development
- Document nix flakew
- Update README.md
- Update README.md
- Update README.md
- Update README.md
- Update README.md
- Readme change screenshot to eza
- Add CoC badge to readme
- Add CODE_OF_CONDUCT.md
- Add crates.io badge, license badge
- Fix links
- Update README.md
- Update README.md

### Documentation

- Add hint how to install exa on Android / Termux

### Git-feature

- Display if a file is updated but unmerged (conflicted)

### Icons

- Add Gentoo for .ebuild

### Io

- :Result -> IOResult

### Src/main.rs

- Remove clippy::unnested_or_patterns

### Vagrant

- Update apt before installing


