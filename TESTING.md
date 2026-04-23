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

## Running the tests

- `just integration-tests` to run the integration tests in docker
- `just unit-tests` to run the unit tests
- or `just test` to run the two commands at once.

## Modifying tests

The integration tests are defined in the `tests` directory.

The `TestDirectory` struct in the `cli_tests_helpers` module provides helper functions to setup a testing environement. Then the crate [trycmd](https://docs.rs/trycmd/) is used to run eza and assert the outputs.

The tests assertions are defined in the `tests/cmd` directory, with the structure `cmd/{platform}/{group}/{test}.toml`.
 - `{platform}` is one of `any` for tests that are not platform specific or `unix` for Unix specific tests.
 - `{group}` represent a group of tests using the same testing environment.

### Regenerating the tests assertions

When changing the behaviour of eza or when adding options, it is necessary to regerate the tests assertions using `just integration-test-regen`.

### Debugging tests

Running the integration tests manually:
```
docker compose run --rm tests cargo test -- --test cli_tests --test-threads 1
```

Forcing the container image to be rebuilt:
```
docker compose run --build --rm tests cargo test -- --test cli_tests --test-threads 1
```


Setting `RUST_BACKTRACE`:
```
docker compose run --rm -e RUST_BACKTRACE=1 tests cargo test -- --test cli_tests --test-threads 1
```

Lauching a shell in the testing environment:
```
docker compose run --rm tests bash
```

Clearing all caches (build, cargo, test data):
```
docker compose down --volumes
```
