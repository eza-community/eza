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
          default = import ./nix/eza.nix { inherit pkgs naersk' buildInputs; };

          check = naersk'.buildPackage {
            inherit buildInputs;
            src = ./.;
            mode = "check";
          };

          test = naersk'.buildPackage {
            inherit buildInputs;
            src = ./.;
            mode = "test";
          };

          clippy = naersk'.buildPackage {
            inherit buildInputs;
            src = ./.;
            mode = "clippy";
          };
        }
        // (import ./nix/trycmd.nix { inherit pkgs naersk' buildInputs; });

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

        checks = {
          pre-commit-check =
            let
              toFilter = [
                "yamlfmt"
                "nixfmt"
                "taplo"
                "shellcheck" # this doesn't respect our excludes:w
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
                convco.enable = true;
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
