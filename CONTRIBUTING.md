# Contributing to eza

If you'd like to contribute to eza, there are several things you should make
sure to familiarize yourself with first.

- Code of conduct [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md)
- Requirement of conformance to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
- Requirement of conformance to [Semantic Versioning](https://semver.org/)
- The [Security Policy](SECURITY.md)
- [Free and Open Source (FOSS) software](https://www.gnu.org/philosophy/free-sw.en.html)

## Hacking on eza

It is strongly recommended that you install Nix for hacking on eza. We leverage
nix  as a way to easily test and distribute eza to many users, and it allows us
to provide multiple tools easily for developers. Instead of having to install
each dependency manually and setting up a development environment, Nix allows
you  to use the same environment as the devs use.

Therefore, it is useful that you have a version of Nix installed with the
"experimental" feature flakes enabled. Further, to make hacking on eza as easy
as possible for yourself, you'd do yourself a favor to install
[direnv](https://direnv.net/).

When you enter the eza repository, if you have `direnv` installed, you'll be
prompted to allow it with `direnv allow`. Doing this will save you from having
to manually enter the development environment each time you open the folder. If
you don't have direnv installed however, you can run `nix develop` in a pinch,
to enter the direnv.

The development environment includes basic checks of conformance to conventional
commits, cargo clippy lints, and much more.

It also contains a pre-commit-hook making it a lot easier not to make potential
mistakes that will unnecessarily delay getting your PRs accepted. Most
importantly, it ensures your commits are conforming to conventional commits.

Some useful commands include:
- `nix flake check`: checks everything is correct.
- `nix build`: build eza.
- `nix build .#test`: runs eza's cargo tests
- `nix build .#clippy`: runs clippy on eza
- `nix fmt`: automatically formats your code as required by flake cheks and
  pre-commit-hooks.nix
- `just itest`: runs integration tests

The [just](https://github.com/casey/just) command runner can be used to run some
helpful development commands, in a manner similar to `make`.  Run `just --list`
to get an overview of what’s available.

To compile the manual pages, you will need [pandoc](https://pandoc.org/), which
the nix flake should have installed for you.  The `just man` command will
compile the Markdown into manual pages, which it will place in the `target/man`
directory.

eza depends on [libgit2](https://github.com/rust-lang/git2-rs) for certain
features.  If you’re unable to compile libgit2, you can opt out of Git support
by running `cargo build --no-default-features`. Again, the nix flake should
have taken care of this for you, if not, please file an issue.

If you intend to compile for musl, you will need to use the flag
`vendored-openssl` if you want to get the Git feature working.  The full command
is `cargo build --release --target=x86_64-unknown-linux-musl --features
vendored-openssl,git`.

If you want more information on the tests please read:
[testing on eza](https://github.com/eza-community/eza/blob/main/TESTING.md)

## Creating a PR

First, use the pull request template.

Please make sure that the thing you worked on... actually works. Make sure to
also add how you ensured this in the PR description. Further, it's expected
that you do your best to check for regressions.

If your PR introduces a flag, you MUST:
- Add completions for bash, zsh, fish, nushell
- Add documentation to the man page
- Add your option to the help flag
- Add your option to the README.md

Before submitting, you SHOULD have run `nix flake check` and ensured that all
issues are addressed. For formatting issues, `nix fmt` will format the code for
you. Most clippy issues can be resolved with `cargo clippy --fix` (although it
might be educational to fix them yourself). If you have reuse issues, you can
run the following command to annotate your code:

Here are the absolute basics:
- your commit summary MUST follow conventional commits.
- your commits SHOULD be separated into small, logical chunks.
- reviewers may ask you to rebase your commits into more sensible chunks.
- your PR will need to pass CI and local `cargo test`.
- you may be asked to refactor parts of your code by reviewers.

Remember that no one here is an employee, and treat everyone with respect, as
the code of conduct specifies. Also remember to be patient if it takes a while
to get a response on your PR. Usually it doesn't, but there's only so many
hours in a day, and if possible, there would be no delay. The delay alone is
evidence of it's own necessity.

## Commit Messages
A common commit message contains at least a summary and reference with
closing action to the corresponding issue if any, and may also include a
description and signature.

### Summary
For you commit messages, please use the first line for a brief summary what
the commit changes. Try to stay within the 72 char limit and prepend what type
of change. See the following list for some guidance:
- feat: adds a new feature to eza
- feat(zsh): adds something to zsh completion
- refactor: revises parts of the code
- docs(readme): revise the README
- docs(man): revision of the man pages
- fix: bugfix in the code base
- fix(ci): bugfix in the continuous integration
- ...

Note that this list is not complete and there may be cases where a commit
could be characterized by different types, so just try to make your best
guess. This spares the maintainers a lot of work when merging your PR.

### Description
If you commit warrants it due to complexity or external information required
to follow it, you should add a more detailed description of the changes,
reasoning and also link external documentation if necessary. This description
should go two lines below the summary and except for links stay in the 80 char
limit.

### Issue Reference
If the commit resolves an issue add: `Resolves: #abc` where `abc` is the issue
number. In case of a bugfix you can also use `Fixes: #abc`.

### Signature
You may add a signature at the end two lines below the description or
issue reference.

### Example
Here is an example of a commit message for a breaking change that follows these rules:

```
fix(hyperlinks)!: TextCell building of detailed grid view, hyperlink, icon options

The hyperlink option adds an escape sequence which in the normal TextCell
creation also becomes part of the length calculation. This patch applies
the same logic the normal grid already did, by using the filenames bare
width when a hyperlink is embedded. It also respects the ShowIcons
option just like the normal grid view.

BREAKING CHANGE: The style codes for huge file and units where
documented to be `nt` and `ut` but the code was using `nh` and `uh`.
The code has been updated to match the documented style codes.
EXA_COLORS using style codes `nh` and `uh` will need to be updated to
use `nt` and `ut`.

Resolves: #129
Ref: #473, #319

Co-authored-by: 9glenda <plan9git@proton.me>
Signed-off-by: Christina Sørensen <christina@cafkafk.com>
```

### Additional Examples

- feat: add column selection
- fix(output): fix width issue with columns
- test(fs): add tests for filesystem metadata
- feat!: breaking change / feat(config)!: implement config file
- chore(deps): update dependencies

### Commit types

- build: Changes that affect the build system or external dependencies (example libgit2)
- ci: Changes to CI configuration files and scripts (example scopes: Nix, Vagrant, Docker)
- chore: Changes which do not change source code or tests (example: changes to the build process, auxiliary tools, libraries)
- docs: Documentation, README, completions, manpage only
- feat: A new feature
- fix: A bug fix
- perf: A code change that improves or addresses a performance issue
- refactor: A code change that neither fixes a bug nor adds a feature
- revert: Revert something
- style: Changes that do not affect the meaning of the code (example: clippy)
- test: Adding missing tests or correcting existing tests
