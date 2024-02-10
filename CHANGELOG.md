# Changelog

## [0.18.2] - 2024-02-08

### Bug Fixes

- Update libgit2 to 1.7.2

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
- Bump DeterminateSystems/nix-installer-action from 4 to 7
- Bump libc from 0.2.149 to 0.2.150
- Bump rustix from 0.38.13 to 0.38.21

### Ci

- Refactor pre-commit-hooks
- Refactor publish workflow

## [0.15.2] - 2023-11-02

### Bug Fixes

- Correct width when --no-quotes is used
- Clippy lint and add option to grid-details
- Changed quote in --almost-all completion
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

- Don’t display target’s size if we’re not dereferencing
- Updated match indents
- Changed flag name
- Only store top-level recursive dir size
- Changed windows methods
- Underscored unused windows variables
- Added device for filesystem to hashmap
- Display offset for filenames with spaces
- Fix clippy warnings
- Fix doc-tests on RecursiveSize
- Fix dead_code warnings on Windows

### Documentation

- Fix doc-tests formatting and address other documentation review requests

### Features

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
- Clippy lint
- Merge conflict with main
- Reverted autofmt changes

### Documentation

- Correct color option spellings
- Added flag to readme
- Added flag to man

### Features

- Add option --smart-group
- Add completions, man for --smart-group
- Added recursive directory parser
- Add icons=always,auto,never. dont display icons in a tty|piped
- Fix auto value for colors and icons + documentation
- Added flag to completions
- [**breaking**] Remove --no-icons in favor of --icons=always,auto,never. default is auto
- Add a new filetype for source code files
- Add a new icons for source code files and other files

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
- Adjust change width calculations for hyperlink and classify
- Root group not painted as expected when eza used by root

### Documentation

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

- Make checksums easier to copy-paste
- Bump trycmd from 0.14.17 to 0.14.19
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

- Merge conflict with main
- Merge conflict with main
- Avoid unstable inner attributes
- Query stdout terminal size to see if the output gose to a tty.
- Use windows-specific API for terminal size query on windows
- Add `windows-sys` dependency for targeting windows
- Fix manpage generation of default package
- Use `std::io::IsTerminal` to eliminate compatibility issue
- Changed dll icon
- Changed readme and Added README icon
- New R lang icon
- Terminal size query should only check `stdout`
- Prefix unused binding name with underscore
- Fix large_enum_variant warning with explanation
- README is sorted and formatted

### Documentation

- Add completions + manpage for --no-quotes flag
- Leave nix install instructions open-ended
- Leave nix install instructions open-ended
- Documenting custom time-style
- Added the new colors option to the man
- Adding termux section
- Time-format supporting custom formats
- Description of `--color` in README, manpage, and completions
- Change `color` to `colo[u]r` in the option description.
- Updated man to add new colors
- Correct CONTRIBUTING.md on commit message type

### Features

- Add quotations around filenames with spaces. exa pr#1165
- Add rustfmt.toml file to prevent flags.rs fmt on save
- [**breaking**] Separated root from other users
- Added statically linked binaries
- Replace hardcoded version by version variable
- Add header to colors-explanation page
- Revise man rule to use for loop and insert version
- New Rust icon
- Added bdf,psf icons
- Added lib icon
- Added Contacts,Favorites icons
- Added home icon
- Added fdmdownload icon
- Adding the possibility to change git-repos colors

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
- Error for missed semicolon
- More than 3 bools in a struct
- Major and minor device on MacOS
- Linux uses u32 for major/minor device numbers
- Respect spec on Windows and make it for with Konsole
- Don’t show color when color is disabled
- Enable rustfmt by removing .rustfmt.toml which disables it
- Replace rustfmt::skip on expressions because experimental
- Remove unnecessary rustfmt::skip's in windows code
- Add src/options/flags.rs to rustfmt.excludes
- Left-over merge conflict in src/output/table
- Ignore refs for blame

### Documentation

- Update README.md
- Update --mounts option to include MacOS
- Documenting --only-files

### Features

- Listing files only using '--only-files' flag
- Add EXA_COLOR bindings for un-themed items
- Add EZA_ environment variables with fallback to EXA_
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

- Test for listing files only
- Add unit tests for new style abbreviations
- Regen git_repos_no_status

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
- Nix flake check also builds the package
- [**breaking**] Change number_huge and unit_huge to match the man page short codes

### Documentation

- Added cafkafk suggestions
- Fix codeblocks in zsh completions
- Update README.md
- Document filetypes theme and rename trait
- Link directly to space
- Add Mac support for the --mount option in the man page
- Add SAFETY comments to unsafe code blocks
- Update deb instructions to use keyring
- Fix chmod in deb installation instructions
- Add potential gpg install to deb installation instructions
- Document character style pairs in the code and match with man page
- Add install instructions for Void Linux
- Documentation of 'sn' and 'sb' conflicted with later docs
- Document dimmed and italic style codes

### Features

- Add completion files in deb packaging script
- Adds filtering for Windows hidden files
- Support --mount option on Mac
- Support --mount option on Mac
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

### Testing

- Stabalised unit-tests.yml
- Autogenerate testing dir
- Autogenerate test dirs
- Generate device files
- Add unit tests that test both exa and ls style codes together
- Address variable names

### Build

- Add musl binary for linux
- Fix checksums
- Add TODOs to targets
- Set optlevel to 3

### Ci

- Add nix Flake check to flake.yml
- Removed nix build in favor of nix flake check
- Include bash completion script in treefmt and fixed shellcheck formatting in completion script
- Fix windows build
- Fix spelling attemps -> attempts

## [0.12.0] - 2023-09-14

### Bug Fixes

- Expand `--all` help
- RUSTSEC-2020-0071
- Generalize gitignore to ignore all eza deb packages
- Canonicalize errors when the destination of a symbolic link is bad
- Handle other canonicalize errors in hyperlinks and git
- Fix windows build when canonicalize returns an error
- Change trycmd config to use test/itest folder for testing
- Revert to old apt install command suggestion and add hint
- Remove stray backslashes
- Is_some_and is an unstable Rust feature until 1.70
- Revert "Support for Windows Hidden Files"
- Shellcheck warnings
- Revert "Support for Windows Hidden Files"
- Shellcheck warnings
- Exit 13 on os error 13
- Rewrite comment
- Improve trace strings
- Tracing typo

### Documentation

- Expand `--all` documentation
- Add pthorpe92 gist
- Remove xtests section from readme
- Add deprecation warning to xtests/readme
- Add deprecation warning to just xtest commands
- Add deprecation warning to vagrantfile
- Add MacPorts install info
- Add gentoo
- Fix gentoo install
- Add docs for --git-repos & --git-repos-no-status
- Fix gpg armor flag for deb release in readme
- Add better explanation of git repos + no status
- Add scoop install info
- Add Winget install info
- Added instructions to install completions of eza to the readme
- Remove color specifications. change unknown git repo status to `~`
- Fix missing color specification from man page
- Remove license from developemnt section
- Update rust badge
- Add missing man page for debian release

### Features

- Add audit workflow
- Add trycmd as dev-dependency
- Add minimal trycmd binary
- Add a few trycmd tests as example
- Document and change output for --git-repos
- Add apt installation workflow
- Adds filtering on Windows hidden files
- Adds filtering on Windows hidden files
- Adds filtering on Windows hidden files
- Added shellcheck to treefmt
- Adds filtering on Windows hidden files
- Add PERMISSION_DENIED exit code

### Miscellaneous Tasks

- Bump chrono from 0.4.27 to 0.4.30
- Removal of xtests
- Removal of vagrant
- Remove deprecated devtools
- [**breaking**] MSRV 1.70
- Run spellcheck
- Release 0.12.0

### Refactor

- Rename FileType::Immediate to more obvious FileType::Build
- Over-engineer deb-package.sh
- Hide xtests folder
- Split trycmd into tests for all, unix and windows
- Limit unit-tests run on workflow change to unit-tests itself
- Moved generateTest.sh to devtools/
- Renamed the file
- Add tracing to various code parts
- Make std::process::exit global

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
- Update flake.lock, cargo.lock
- Add deny.toml
- Remove org warnings
- Remove itest
- Update flake.lock
- Add itest, idump
- Make trycmd part of checks

### Ci

- Don't use nix feature on ci
- Add Winget Releaser workflow
- 1.65 -> 1.70
- Enforce conventional commits
- Enforce conventional commits

## [0.11.1] - 2023-09-11

### Bug Fixes

- Add vendored-libgit2 feature to git2 dependency
- Filename escaping (last character lost sometimes, no hyperlink)
- Build for Windows with chrono
- Needless_borrow

### Documentation

- Explain vendored-libgit2
- Add homebrew, misc fixes
- Fix code of conduct link
- Update archlinux
- Add star history
- Add informaton about lazy_static
- Remove broken dependabot link
- Add bright color options in man pages
- Add bright color support in readme changelog
- Document new file type two letter codes in man page

### Features

- Add highlighting of mounted directories (Linux only)
- Add backlog of icons from various exa pull requests and others
- Mark `.git` as ignored, which hides it when using `--git-ignore`
- Add backlog of icons from various exa issues
- Expose git2 feature vendored-libgit2
- Add build commands to deb-package.sh
- Support the MSRV of Rust (1.65.0)
- Add bright colour options, change punctuation default
- Use chrono crate to handle datetime-related features
- Make file types themeable

### Miscellaneous Tasks

- Bump actions/checkout from 3 to 4
- Bump uzers to v0.11.3
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

### Miscellaneous Tasks

- Bump git2 from 0.17.2 to 0.18.0
- Bump uzers from 0.11.1 to 0.11.2
- Bump glob from 0.3.0 to 0.3.1
- Bump DeterminateSystems/nix-installer-action from 3 to 4
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

- Bump DeterminateSystems/flake-checker-action from 4 to 5
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

### Deps

- Change users depedency to uzers

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
- Add git-ignore style/color information to manpage
- --blocksize, new description
- --blocksize, new description
- --blocksize, new description
- Cafkafk -> eza-community
- Add gpg public key for the deb repository
- Add section about debian and ubuntu installation
- Add guidelines for commit messages

### Features

- Add `just` and `pandoc` to devShell bc they are necessary for man
- Add `.envrc` so direnv automatically opens the nix dev environment
- Add git-ignored color/style option
- Match folder icon to reflect contents
- Match folder icon to reflect contents
- --blocksize completion, new description
- Add script deb-package.sh

### Miscellaneous Tasks

- Bump libc from 0.2.93 to 0.2.147
- Bump num_cpus from 1.13.0 to 1.16.0
- Bump git2 from 0.16.1 to 0.17.2
- Bump unicode-width from 0.1.8 to 0.1.10
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

### Git

- Add deb package to .gitignore

## [0.10.7] - 2023-08-13

### Bug Fixes

- Broken zsh completion syntax
- Respect GIT_CEILING_DIRECTORIES
- MacOS flake support

### Documentation

- Create SECURITY.md
- Create CONTRIBUTING.md

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
- Bump actions/stale from 5 to 8
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

## [0.10.6] - 2023-08-07

### Bug Fixes

- Rename eza-colors-explanation
- Exa -> eza in manpage

### Documentation

- Adding --git-repos to help.
- Add aur, nixpkgs installation

### Features

- Use GIT_DIR env var to find the repo
- Add color explanations

### Miscellaneous Tasks

- Release 0.10.6

## [0.10.5] - 2023-08-03

### Bug Fixes

- Output wraps in terminal
- Respect icon spacing

### Miscellaneous Tasks

- Release 0.10.5

## [0.10.4] - 2023-08-02

### Bug Fixes

- Dereferencing linksfile size.
- Dereferencing links users.
- Dereferencing links groups.
- Dereferencing links permissions.
- Dereferencing links timestamps.
- Syntax error

### Documentation

- Add -X/--dereference flag

### Features

- Add symlink dereferencing flag
- Add -X/--dereference completions
- Add -X/--dereference completions
- Added ".out" files for latex
- Add changelog generation

### Miscellaneous Tasks

- Release 0.10.4

## [0.10.3] - 2023-07-31

### Bug Fixes

- More JPG extensions
- Add compression icon to .tXX files #930
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
- .ipynb icon comment
- Better license icon
- Replace obsolete icons
- Add Svelte icon
- Add Emacs icon for .el and org-mode for .org
- Added icons for .rmeta
- Add icon support for .mjs, .cjs, .mts, .cts files
- Add webpack.config.cjs to immediate files list
- Removed result
- Update --version info
- Update snapscraft.yaml
- Sort is_immediate
- Add flake, autoconf, cargo lock
- Added trailing commas
- Remove accidentally commited test files

### Documentation

- Add hint how to install exa on Android / Termux
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

### Feat

- Add JPF to image filetype

### Features

- Add support Typescript and ReasonML projects
- New Icons and CLI argument to suppress icons
- Add sty file
- Add julia file extension icon
- Add status for git repos
- Add selinux contexts support
- Add -o shorcut to --octal-permissions
- Hyperlink flag
- Update Cargo.toml to optimise binaries for size
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
- Symlinks report their target's valid size
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

### Git

- Use GIT_DIR env var to find the repo
- Use open_from_env before discover

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


