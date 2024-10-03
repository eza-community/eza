# SPDX-FileCopyrightText: 2024 Christina Sørensen
# SPDX-License-Identifier: EUPL-1.2
#
# SPDX-FileCopyrightText: 2014-2024 Christina Sørensen, eza contributors
# SPDX-License-Identifier: MIT
{
  description = "eza: a modern, maintained replacement for ls";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs = {
        systems.follows = "systems";
      };
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    powertest = {
      url = "github:eza-community/powertest";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        naersk.follows = "naersk";
        treefmt-nix.follows = "treefmt-nix";
        rust-overlay.follows = "rust-overlay";
      };
    };

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };
  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      treefmt-nix,
      rust-overlay,
      powertest,
      pre-commit-hooks,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs .config/treefmt.nix;

        darwinBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
          pkgs.darwin.apple_sdk.frameworks.Security
        ];

        buildInputs = [ pkgs.zlib ] ++ darwinBuildInputs;
      in
      rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          # For `nix build` `nix run`, & `nix profile install`:
          default = naersk'.buildPackage rec {
            pname = "eza";
            version = "git";

            src = ./.;
            doCheck = true; # run `cargo test` on build

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
          };

          # Run `nix build .#check` to check code
          check = naersk'.buildPackage {
            src = ./.;
            mode = "check";
            inherit buildInputs;
          };

          # Run `nix build .#test` to run tests
          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            inherit buildInputs;
          };

          # Run `nix build .#clippy` to lint code
          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            inherit buildInputs;
          };

          # Run `nix build .#trycmd` to run integration tests
          trycmd = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            doCheck = true;
            # No reason to wait for release build
            release = false;
            # buildPhase files differ between dep and main phase
            singleStep = true;
            # generate testing files
            buildPhase = ''
              bash devtools/dir-generator.sh tests/test_dir && echo "Dir generated"
              bash devtools/generate-timestamp-test-dir.sh tests/timestamp_test_dir
            '';
            cargoTestOptions = opts: opts ++ [ "--features nix" ];
            inherit buildInputs;
            nativeBuildInputs = with pkgs; [ git ];
          };

          # TODO: add conditionally to checks.
          # Run `nix build .#trycmd` to run integration tests
          trycmd-local = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            doCheck = true;
            # No reason to wait for release build
            release = false;
            # buildPhase files differ between dep and main phase
            singleStep = true;
            # set itests files creation date to unix epoch
            buildPhase = ''
              bash devtools/dir-generator.sh tests/test_dir
              bash devtools/generate-timestamp-test-dir.sh tests/timestamp_test_dir
              touch --date=@0 tests/itest/*
              touch --date=@0 tests/ptests/*;
              fd -e stdout -e stderr -H -t file -X sed -i 's/[CWD]\//\/build\/source\//g'
            '';
            cargoTestOptions =
              opts:
              opts
              ++ [
                "--features nix"
                "--features nix-local"
                "--features powertest"
              ];
            inherit buildInputs;
            nativeBuildInputs = with pkgs; [ git ];
          };

          # Run `nix build .#trydump` to dump testing files
          trydump = naersk'.buildPackage rec {
            src = ./.;
            mode = "test";
            doCheck = true;
            # No reason to wait for release build
            release = false;
            # buildPhase files differ between dep and main phase
            singleStep = true;
            # set itests files creation date to unix epoch
            buildPhase = ''
              bash devtools/dir-generator.sh tests/test_dir
              bash devtools/generate-timestamp-test-dir.sh tests/timestamp_test_dir
              touch --date=@0 tests/itest/*;
              rm tests/cmd/*.stdout || echo;
              rm tests/cmd/*.stderr || echo;

              touch --date=@0 tests/ptests/*;
              rm tests/ptests/*.stdout || echo;
              rm tests/ptests/*.stderr || echo;
            '';
            cargoTestOptions =
              opts:
              opts
              ++ [
                "--features nix"
                "--features nix-local"
                "--features powertest"
                #"-F trycmd/debug"
              ];
            TRYCMD = "dump";
            postInstall = ''
              fd -e stdout -e stderr -H -t file -X sed -i 's/\/build\/source\//[CWD]\//g'

              cp dump $out -r
            '';
            inherit buildInputs;
            nativeBuildInputs = with pkgs; [
              fd
              gnused
              git
            ];
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          nativeBuildInputs =
            with pkgs;
            [
              # cargo
              # clippy
              rustup
              toolchain
              just
              pandoc
              convco
              zip
              reuse

              # For releases
              b3sum
              cargo-bump

              # For generating demo
              vhs

              powertest.packages.${pkgs.system}.default

              cargo-hack
              cargo-udeps
              cargo-outdated
            ]
            ++ darwinBuildInputs;
        };

        # For `nix flake check`
        checks = {
          pre-commit-check =
            let
              # some treefmt formatters are not supported in pre-commit-hooks we
              # filter them out for now.
              toFilter = [
                "yamlfmt"
                "nixfmt"
                "taplo"
              ];
              filterFn = n: _v: (!builtins.elem n toFilter);
              treefmtFormatters = pkgs.lib.mapAttrs (_n: v: { inherit (v) enable; }) (
                pkgs.lib.filterAttrs filterFn (import .config/treefmt.nix).programs
              );
            in
            pre-commit-hooks.lib.${system}.run {
              src = ./.;
              hooks = treefmtFormatters // {
                nixfmt-rfc-style.enable = true;
                convco.enable = true; # not in treefmt
                reuse = {
                  enable = true;
                  name = "reuse";
                  entry = with pkgs; "${reuse}/bin/reuse lint";
                  pass_filenames = false;
                };
              };
            };
          formatting = treefmtEval.config.build.check self;
          build = packages.check;
          inherit (packages)
            default
            test
            trycmd
            ;
          lint = packages.clippy;
        };
      }
    );
}
