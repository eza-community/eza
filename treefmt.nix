{
  projectRootFile = "Cargo.toml";
  programs = {
    alejandra.enable = true; # nix
    statix.enable = true; # nix static analysis
    deadnix.enable = true; # find dead nix code
    rustfmt.enable = true; # rust
    shellcheck.enable = true; # bash/shell
    taplo.enable = true; # toml
    yamlfmt.enable = true; # yaml
  };
  settings = {
    formatter = {
      shellcheck.includes = ["*.sh" "./completions/bash/eza"];
      rustfmt.excludes = ["src/options/flags.rs"];
      taplo.excludes = ["tests/ptests/*.toml"];
      yamlfmt.excludes = ["./powertest.yaml"];
    };
  };
}
