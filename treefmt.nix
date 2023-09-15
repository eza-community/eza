{
  projectRootFile = "Cargo.toml";
  programs = {
    alejandra.enable = true;
    rustfmt.enable = true;
    shellcheck.enable = true;
  };
  settings = {
    formatter.shellcheck.includes = ["*.sh" "./completions/bash/eza"];
  };
}
