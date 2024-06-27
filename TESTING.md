# Testing eza

## Running tests

In order to run the tests in eza you need:
- [just](https://github.com/casey/just)
- [nix](https://nixos.org)

then either run:
- `just itest`
- `nix build -L trycmd-local`

## Modifying tests

In order to test your changes on eza, you will need to do one or multiple things in different cases.
You will need the additional tool
- [powertest](https://github.com/eza-community/powertest)

You will also need to modify the `devtools/dir-generator.sh` file if you want to add some test cases

### You added/modified an option

Please run `just regen` to regenerate powertesting. Then look into `tests/gen` or `tests/cmd` for any tests not passing

### You changed the output of eza

Please run `nix build -L trydump` or `just idump`
And lookout for any test no longer passing
