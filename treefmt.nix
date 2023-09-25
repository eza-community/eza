{
  projectRootFile = "Cargo.toml";
  programs = {
    alejandra.enable = true; # nix
    rustfmt.enable = true; # rust
    shellcheck.enable = true; # bash/shell
    deadnix.enable = true; # find dead nix code
    taplo.enable = true; # toml
    yamlfmt.enable = true; # yaml
  };
  settings = {
    formatter.shellcheck.includes = ["*.sh" "./completions/bash/eza"];
    formatter.rustfmt.excludes = ["src/options/flags.rs"];
  };
}
