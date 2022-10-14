{
  description = "Charon";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay, crane }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.nightly."2022-01-29".default.override {
          extensions = [ "rust-src" "rustc-dev" "llvm-tools-preview" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        charon =
          craneLib.buildPackage { src = ./charon; };
        tests = let
          deps =
            craneLib.buildDepsOnly { src = ./tests; };
        in pkgs.stdenv.mkDerivation {
          name = "tests";
          src = ./tests;
          nativeBuildInputs = [ pkgs.gnutar pkgs.zstd rustToolchain ];
          configurePhase = ''
            tar -xf ${deps}/target.tar.zst
            mkdir -p $out/llbc
          '';
          buildPhase = ''
            CHARON="${charon}/bin/cargo-charon --release --cargo-no-rust-version --dest $out/llbc"

            for test in hashmap matches matches_duplicate
            do
              $CHARON --crate $test --input src/$test.rs
            done

            for test in nested_borrows no_nested_borrows loops paper constants external
            do
              $CHARON --no-code-duplication --crate $test --input src/$test.rs
            done

            $CHARON --opaque=hashmap_utils --crate hashmap_main --input src/hashmap_main.rs
          '';
          dontInstall = true;
        };
        tests-polonius = let
          deps = craneLib.buildDepsOnly {
            src = ./tests-polonius;
          };
        in pkgs.stdenv.mkDerivation {
          name = "tests-polonius";
          src = ./tests-polonius;
          nativeBuildInputs = [ pkgs.gnutar pkgs.zstd rustToolchain ];
          configurePhase = ''
            tar -xf ${deps}/target.tar.zst
            mkdir -p $out/llbc
          '';
          buildPhase = ''
            CHARON="${charon}/bin/cargo-charon --release --cargo-no-rust-version --dest $out/llbc"

            $CHARON --opaque=betree_utils --crate betree_main --input src/betree_main.rs
          '';
          dontInstall = true;
        };
      in {
        packages = {
          inherit charon;
          default = charon;
        };
        hydraJobs = { inherit charon tests tests-polonius; };
      });
}
