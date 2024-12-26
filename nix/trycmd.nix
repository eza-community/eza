# SPDX-FileCopyrightText: 2024 Christina Sørensen
# SPDX-License-Identifier: EUPL-1.2
{
  pkgs,
  naersk',
  buildInputs,
  ...
}:

{
  trycmd = naersk'.buildPackage {
    src = ../.;
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
    src = ../.;
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
  trydump = naersk'.buildPackage {
    src = ../.;
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
}
