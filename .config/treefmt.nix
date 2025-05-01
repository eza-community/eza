# SPDX-FileCopyrightText: 2024 Christina Sørensen
# SPDX-License-Identifier: EUPL-1.2
{
  projectRootFile = "Cargo.toml";
  programs = {
    nixfmt.enable = true; # nix
    statix.enable = true; # nix static analysis
    deadnix.enable = true; # find dead nix code
    # TODO https://github.com/numtide/treefmt-nix/issues/343
    #rustfmt.enable = true; # rust
    shellcheck.enable = true; # bash/shell
    taplo.enable = true; # toml
    yamlfmt.enable = true; # yaml
  };
  settings = {
    formatter = {
      shellcheck = {
        includes = [
          "*.sh"
          "./completions/bash/eza"
        ];
        excludes = [ ".envrc" ];
      };
      #rustfmt.excludes = [ "src/options/flags.rs" ];
      taplo.excludes = [ "tests/ptests/*.toml" ];
      yamlfmt.excludes = [ "./powertest.yaml" ];
    };
  };
}
