# Changelog

All notable changes to this project will be documented in this file.

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
- Remove color specifications. change unknown git repo status to `~`
- Fix missing color specification from man page

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
- Run spellcheck

### Refactor

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
- Enforce conventional commits
- Enforce conventional commits

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
- Add star history
- Add informaton about lazy_static
- Remove broken dependabot link
- Add bright color options in man pages
- Add bright color support in readme changelog

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

### Features

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

### Improve

- Vim icon

### Miscellaneous Tasks

- Bump to v0.10.2
- Bump to v0.10.3
- Update cargo lock

### Refactor

- Removed commented code
- Sorted file types, color table

### Styling

- Add icon for reStructuredText (src) files

### Testing

- Change to /usr/bin/env bash

### Add

- Mp2 audio format icon

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

### Git

- Use GIT_DIR env var to find the repo
- Use open_from_env before discover

### Icons

- Add Gentoo for .ebuild

### Src/main.rs

- Remove clippy::unnested_or_patterns

## [0.10.0] - 2021-04-03

### Documentation

- Add hint how to install exa on Android / Termux

### Features

- Add support Typescript and ReasonML projects
- New Icons and CLI argument to suppress icons

### Miscellaneous Tasks

- Update zoneinfo_compiled, datetime to 0.5
- Update users to 0.10
- PR feedback

### Build

- Use binary name only

### Git-feature

- Display if a file is updated but unmerged (conflicted)

## [0.9.0] - 2019-07-15

### Feat

- Add JPF to image filetype

### Refactor

- Use shorthand fields

## [0.8.0] - 2017-09-30

### Vagrant

- Update apt before installing

## [0.4.1] - 2017-03-26

### Fixup

- Split prefix tests by property

### Io

- :Result -> IOResult

## [0.4.0] - 2015-10-18

### Makefile

- Be compatible with BSD and OS X

## [0.3.0] - 2015-06-05

### StatResult

- :Path -> Dir

## [0.2.0] - 2015-03-02

### Details

- `filter` is only used when recursing

## [0.1.0] - 2015-02-21

### ToStr

- :to_str -> ToString::to_string

<!-- generated by git-cliff -->
