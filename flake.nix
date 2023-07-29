{

  inputs = {

    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk";

    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    treefmt-nix.url = "github:numtide/treefmt-nix";

    rust-overlay.url = "github:oxalica/rust-overlay";

  };

  outputs = {
      self
    , flake-utils
    , naersk
    , nixpkgs
    , treefmt-nix
    , rust-overlay
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

        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;

      in {

        # For `nix fmt`

        formatter = treefmtEval.config.build.wrapper;

        # For `nix build` & `nix run`:

        packages.default = naersk'.buildPackage {

          src = ./.;

        };

        # For `nix develop`:

        devShells.default = pkgs.mkShell {

          nativeBuildInputs = with pkgs; [toolchain];

        };

        # for `nix flake check`

        checks = {

          formatting = treefmtEval.config.build.check self;

        };

      }

    );

}


