# Changelog

All notable changes to this project will be documented in this file.

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
