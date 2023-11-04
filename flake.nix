{
  description = "eza: a modern, maintained replacement for ls";

  inputs = {
    flake-utils = {
      url = "http://rime.cx/v1/github/semnix/flake-utils.tar.gz";
    };
    flake-compat.url = "github:edolstra/flake-compat";
    nixpkgs.url = "http:/rime.cx/v1/github/NixOS/nixpkgs/b/nixpkgs-unstable.tar.gz";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "http://rime.cx/v1/github/semnix/rust-overlay.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "http://rime.cx/v1/github/semnix/treefmt-nix.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # for crane cargoAudit
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

  outputs = inputs @ {
    treefmt-nix,
    flake-parts,
    rust-overlay,
    pre-commit-hooks,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.pre-commit-hooks.flakeModule
      ];
      # same as flake-utils.eachDefaultSystem: https://github.com/nix-systems/default/blob/main/default.nix
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      perSystem = {
        pkgs,
        lib,
        system,
        config,
        ...
      }: let
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

        src = lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (lib.hasInfix "/man/" path)
            || (lib.hasInfix "/completions/" path)
            || (lib.hasInfix "/tests/" path)
            || (craneLib.filterCargoSources path type);
        };

        commonArgs = {
          inherit src;
          buildInputs = with pkgs; [zlib] ++ lib.optionals stdenv.isDarwin [libiconv darwin.apple_sdk.frameworks.Security];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        metaFromCargoToml = cargoToml:
          with pkgs.lib; {
            inherit (cargoToml.package) description;
            longDescription = cargoToml.package.metadata.deb.extended-description;
            inherit (cargoToml.package) homepage;
            downloadPage = cargoToml.package.repository;
            mainProgram = lib.warnIf (builtins.length cargoToml.bin > 1) "Cargo.toml has more then one binary in [[bin]]. This can cause issues in the `metaFromCargoToml` function" (builtins.head cargoToml.bin).name;
            license = lib.getLicenseFromSpdxId cargoToml.package.license;
          };
        fromCargoToml = cargoToml: {
          pname = cargoToml.package.name;
          version = "${cargoToml.package.version}-git";
        };
      in rec {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };
        treefmt = import ./treefmt.nix;

        packages = rec {
          default = eza;
          eza = craneLib.buildPackage (commonArgs
            // rec {
              inherit cargoArtifacts src;
              inherit (fromCargoToml cargoToml) pname version;

              nativeBuildInputs = with pkgs; [cmake pkg-config installShellFiles pandoc];

              buildNoDefaultFeatures = true;
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
              meta =
                metaFromCargoToml cargoToml
                // {
                  maintainers = with pkgs.lib.maintainers; [cafkafk];
                };
            });

          eza-trydump = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts src;
              buildPhase = ''touch --date=@0 tests/itest/*; rm tests/cmd/*.stdout || echo; rm tests/cmd/*.stderr || echo;'';
              cargoExtraArgs = "--features=nix,nix-local";
              partitions = 1;
              partitionType = "count";
              postInstall = ''
                cp dump $out -r
              '';
              TRYCMD = "dump";
            });
          eza-trycmd-local = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts src;
              buildPhase = ''                echo "changing date to @0"
                              touch --date=@0 tests/itest/*'';
              cargoExtraArgs = "--features=nix,nix-local";
              partitions = 1;
              partitionType = "count";
            });
          eza-trycmd = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts src;
              buildPhase = ''                echo "changing date to @0"
                              touch --date=@0 tests/itest/*'';
              cargoExtraArgs = "--features=nix";
              partitions = 1;
              partitionType = "count";
            });
          eza-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts src;
              cargoExtraArgs = "--color=never";
              partitions = 1;
              partitionType = "count";
            });
          eza-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts src;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });
          eza-audit = craneLib.cargoAudit {
            inherit src;
            inherit (inputs) advisory-db;
          };
          eza-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts src;
            });
        };

        devShells.default = pkgs.mkShell {
          # inherit (self'.checks.${system}.pre-commit-check) shellHook;
          shellHook = ''
            ${config.pre-commit.installationScript}
            echo 1>&2 "eza development shell!"
          '';
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
            cargo-nextest
          ];
        };
        pre-commit = let
          # some treefmt formatters are not supported in pre-commit-hooks we filter them out for now.
          toFilter =
            # This is a nice hack to not have to manually filter we should keep in mind for a future refactor.
            # (builtins.attrNames pre-commit-hooks.packages.${system})
            ["yamlfmt"];
          filterFn = n: _v: (!builtins.elem n toFilter);
          treefmtFormatters = pkgs.lib.mapAttrs (_n: v: {inherit (v) enable;}) (pkgs.lib.filterAttrs filterFn (import ./treefmt.nix).programs);
        in {
          settings = {
            src = ./.;

            hooks =
              treefmtFormatters
              // {
                convco.enable = true; # not in treefmt
              };
          };
        };
        # For `nix flake check`
        checks = {
          inherit
            (packages)
            eza-doc
            eza-clippy
            eza-audit
            eza-nextest
            eza-trycmd
            ;
          # formatting = treefmtEval.config.build.check self;
          # build = packages.check;
          # default = packages.default;
          # test = packages.test;
          # lint = packages.clippy;
          # trycmd = packages.trycmd;
        };
      };
    };
}
