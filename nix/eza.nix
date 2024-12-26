# SPDX-FileCopyrightText: 2024 Christina Sørensen
# SPDX-License-Identifier: EUPL-1.2
{
  pkgs,
  naersk',
  buildInputs,
  ...
}:

naersk'.buildPackage rec {
  pname = "eza";
  version = "git";

  src = ../.;
  doCheck = true;

  inherit buildInputs;
  nativeBuildInputs = with pkgs; [
    cmake
    pkg-config
    installShellFiles
    pandoc
  ];

  buildNoDefaultFeatures = true;
  buildFeatures = "git";

  postInstall = ''
    for page in eza.1 eza_colors.5 eza_colors-explanation.5; do
      sed "s/\$version/${version}/g" "man/$page.md" |
        pandoc --standalone -f markdown -t man >"man/$page"
    done
    installManPage man/eza.1 man/eza_colors.5 man/eza_colors-explanation.5
    installShellCompletion \
      --bash completions/bash/eza \
      --fish completions/fish/eza.fish \
      --zsh completions/zsh/_eza
  '';

  meta = with pkgs.lib; {
    description = "A modern, maintained replacement for ls";
    longDescription = ''
      eza is a modern replacement for ls. It uses colours for information by
      default, helping you distinguish between many types of files, such as
      whether you are the owner, or in the owning group. It also has extra
      features not present in the original ls, such as viewing the Git status
      for a directory, or recursing into directories with a tree view. eza is
      written in Rust, so it’s small, fast, and portable.
    '';
    homepage = "https://github.com/eza-community/eza";
    license = licenses.mit;
    mainProgram = "eza";
    maintainers = with maintainers; [ cafkafk ];
  };
}
