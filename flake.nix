{
  description = "The EZA flake for developing and releasing (soon)";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
    };
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-compat.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # for crane cargoAudit
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    flake-utils,
    crane,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        inherit (pkgs) lib;

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        # Rust version and which components to install.
        # We use the toolchain from ./rust-toolchain.toml in order to remain compatible with rustup.
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        # The checks only require the rust source code.
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # The eza package requires aditional folders like `/man` and `/completions`.
        srcForEzaPackage = lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (lib.hasInfix "/man/" path)
            || (lib.hasInfix "/completions/" path)
            || (craneLib.filterCargoSources path type);
        };

        commonArgs = {
          inherit src;
          buildInputs = with pkgs; [zlib] ++ lib.optionals stdenv.isDarwin [libiconv darwin.apple_sdk.frameworks.Security];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        eza = craneLib.buildPackage (commonArgs
          // rec {
            inherit cargoArtifacts;

            src = srcForEzaPackage; # overwrite the src to also include the man folder.
            pname = "eza";
            version = "latest";

            nativeBuildInputs = with pkgs; [cmake pkg-config installShellFiles pandoc];

            buildNoDefaultFeatures = true;
            # buildFeatures = lib.optional gitSupport "git";
            buildFeatures = "git";

            doNotLinkInheritedArtifacts = true;
            postInstall = ''
              pandoc --standalone -f markdown -t man <(cat "man/eza.1.md" | sed "s/\$version/${version}/g") > man/eza.1
              pandoc --standalone -f markdown -t man <(cat "man/eza_colors.5.md" | sed "s/\$version/${version}/g") > man/eza_colors.5
              pandoc --standalone -f markdown -t man <(cat "man/eza_colors-explanation.5.md" | sed "s/\$version/${version}/g")> man/eza_colors-explanation.5
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
              maintainers = with maintainers; [cafkafk];
            };
          });
      in rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        # For `nix build`
        packages = {
          default = eza;
          trydump = craneLib.cargoNextest (commonArgs
            // {
              # include test files
              src = ./.;
              inherit cargoArtifacts;
              mode = "test";
              doCheck = true;
              # No reason to wait for release build
              release = false;
              # buildPhase files differ between dep and main phase
              singleStep = true;
              # set itests files creation date to unix epoch
              buildPhase = ''touch --date=@0 tests/itest/*; rm tests/cmd/*.stdout || echo; rm tests/cmd/*.stderr || echo;'';
              cargoTestOptions = ["--features nix"];
              TRYCMD = "dump";
              postInstall = ''
                cp dump $out -r
              '';
            });

          clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          trycmd = craneLib.cargoNextest (commonArgs
            // {
              # include test files
              src = ./.;
              inherit cargoArtifacts;
              mode = "test";
              doCheck = true;
              # No reason to wait for release build
              release = false;
              # buildPhase files differ between dep and main phase
              singleStep = true;
              # set itests files creation date to unix epoch
              buildPhase = ''touch --date=@0 tests/itest/*'';
              cargoTestOptions = ["--features nix"];
            });
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustup toolchain just pandoc convco];
        };

        # For `nix run`
        apps.default = flake-utils.lib.mkApp {
          drv = eza;
        };

        # For `nix flake check`
        checks = {
          formatting = treefmtEval.config.build.check self;
          # security
          cargo-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
          # lint
          inherit (packages) clippy;
          inherit (packages) trycmd;
          # like `cargo test` but using nextest which is faster.
          cargo-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        };
      }
    );
}
