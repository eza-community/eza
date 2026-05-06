<!--
SPDX-FileCopyrightText: 2024 Christina Sørensen, Martin Fillon
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->
# Testing eza

## Testing environment

In order to run the tests in eza you need:
- [just](https://github.com/casey/just)
- [docker](https://docs.docker.com/desktop/), including [docker-compose](https://docs.docker.com/compose/install) and optionally [docker-buildx](https://github.com/docker/buildx)

Tests will be run on the CI anyway, but running tests yourself may be more practical.

## Running tests

- `just test`: run all unit tests
- `just unit-tests`: runs unit tests
- `just cli-tests`: runs cli tests (both on your machine and in Docker)
- `just cli-tests-local`: runs cli tests (on your machine)
- `just cli-tests-docker`: runs cli tests (on Linux in Docker)
- `just cli-test-regen`: regen output of cli tests (both on your machine and in
  Docker)

Linux-only tests are only run in Docker.

If you’re on Linux, you can only run `just cli-tests-docker` to avoid running twice the tests that are not linux-only.

If you’re not Linux, using `just cli-tests` ensures your changes work on both your OS and Linux.

### Regenerating tests assertions

When changing the behaviour of eza or when adding options, it is necessary to regerate the tests assertions using `just cli-tests-regen` (or `just cli-tests-docker-regen` if you’re on Linux).

## Adding or modifying tests

The cli tests are defined in the `tests` directory.

- `tests/cli_tests_helpers.rs`: provides the `TestDirectory` struct, with helper functions to setup a testing environment, which then the crate [trycmd](https://docs.rs/trycmd/) is used to run eza and assert the outputs.
- `tests/cli_tests_{platform}.rs`: run tests assertions. Functions are in alphabetical order the file, so they are in the same order as the directory they reference in the filesystem.

Cli tests are functions of the form:

```rust
#[test]
fn cli_tests_{platform}_{group}() {
    let test_dir = TestDirectory::new("{platform}", "{group}");

    todo!()
}
```

Tests are automatically run at then end of the function, when `test_dir` is dropped.

The tests assertions are defined in the `tests/cmd` directory, with the structure `cmd/{platform}/{group}/{test}.toml`.

- `{platform}` can be:
    - `any` for tests run on all platforms
    - `linux_docker` for tests run on Linux in Docker
    - `unix`, `macos`, `windows`: for tests run on these specific OSes or family of OSes
    - we don’t have *BSD-specific tests yet, but we could since we have some of these in the CI
- `{group}` represent a group of tests using the same testing environment, named with a few keywords from more important to less important, separated `_`, describing the setup or the point of the group of tests.
- `{test}` describes the flags used in the tests, separated by `_`. Some conventions:
    - starts with `no-flag` when no flag is used / tested
    - doesn’t include `long` (the vast majority of the time) if it’s needed by other tested flags
    - ends with `color` when `--color=always` is used
    - ends with `env-{name}-{value}` in lowercase (after the flags), simplified if necessary. For example:
        - `env-eza-icon-spacing-2` for `EZA_ICON_SPACING = "2"`
        - `env-override-git` for `EZA_OVERRIDE_GIT = "1"` (don’t put `1` in the name because the value is irrelevant)
        - `env-time-fr` for `LC_TIME = "fr_FR.UTF-8"` (often, simpler is better)
    - ends with `fail`, at the very end, if the test is supposed to fail

A test assertion looks like that:

```toml
args = ["--oneline", "."]
status.code = 0  # optional, only put if status code is non-zero
stdout = """
dir
file.txt
"""
stderr = ""

[env.add]
EXAMPLE_ENV_VAR = "value"
```

Most of it is self-explanatory. A few notes:

- `"."` is mandatory, otherwise eza thinks it should read its stdin. Always keep at the end
- use long flags (makes it easier to review)
- `stdout`, `stderr`, and `status.code` are automatically filled/changed when using `just cli-test-regen`
- order of the flags (in general it should in the same order than in the `--help` or the columns it affects):
    - first flags related to display (flag other than long, then `--long`, then `--accross`)
    - the flags of the feature that you’re testing
    - `"--color=always"` if you test the color output
    - if you have to use these flags, `"--no-permissions", "--no-filesize", "--no-user", "--no-time"` should be in this order
    - then `"."`

### Debugging tests

Lauching a shell in the testing environment:
```sh
just cli-tests-docker-shell
```

Clearing all caches (build, cargo, test data):
```sh
docker compose down --volumes
```
