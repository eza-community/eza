{
  description = "eza: a modern, maintained replacement for ls";

  inputs = {
    nixpkgs.url = "http:/rime.cx/v1/github/NixOS/nixpkgs/b/nixpkgs-unstable.tar.gz";

    flake-utils = {
      url = "http://rime.cx/v1/github/semnix/flake-utils.tar.gz";
    };

    naersk = {
      url = "http://rime.cx/v1/github/semnix/naersk.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "http://rime.cx/v1/github/semnix/rust-overlay.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "http://rime.cx/v1/github/semnix/treefmt-nix.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "http://rime.cx/v1/github/semnix/pre-commit-hooks.nix.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
    pre-commit-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        buildInputs = with pkgs; [zlib] ++ lib.optionals stdenv.isDarwin [libiconv darwin.apple_sdk.frameworks.Security];
      in rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          # For `nix build` `nix run`, & `nix profile install`:
          default = naersk'.buildPackage rec {
            pname = "eza";
            version = "latest";

            src = ./.;
            doCheck = true; # run `cargo test` on build

            inherit buildInputs;
            nativeBuildInputs = with pkgs; [cmake pkg-config installShellFiles pandoc];

            buildNoDefaultFeatures = true;
            buildFeatures = "git";

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
            # set itests files creation date to unix epoch
            buildPhase = ''touch --date=@0 tests/itest/*'';
            cargoTestOptions = opts: opts ++ ["--features nix"];
            inherit buildInputs;
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
            buildPhase = ''touch --date=@0 tests/itest/*'';
            cargoTestOptions = opts: opts ++ ["--features nix" "--features nix-local"];
            inherit buildInputs;
          };

          # Run `nix build .#trydump` to dump testing files
          trydump = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            doCheck = true;
            # No reason to wait for release build
            release = false;
            # buildPhase files differ between dep and main phase
            singleStep = true;
            # set itests files creation date to unix epoch
            buildPhase = ''touch --date=@0 tests/itest/*; rm tests/cmd/*.stdout || echo; rm tests/cmd/*.stderr || echo;'';
            cargoTestOptions = opts: opts ++ ["--features nix" "--features nix-local"];
            TRYCMD = "dump";
            postInstall = ''
              cp dump $out -r
            '';
            inherit buildInputs;
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          nativeBuildInputs = with pkgs; [
            toolchain
            rustup
            just
            pandoc
            convco
            zip

            cargo-hack
            cargo-udeps
            cargo-outdated
          ];
        };

        # For `nix flake check`
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              convco.enable = true;
              alejandra.enable = true;
              deadnix.enable = true;
              rustfmt.enable = true;
              shellcheck.enable = true;
              taplo.enable = true;
            };
          };
          formatting = treefmtEval.config.build.check self;
          build = packages.check;
          default = packages.default;
          test = packages.test;
          lint = packages.clippy;
          trycmd = packages.trycmd;
        };
      }
    );
}
