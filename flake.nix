{
  description = "PhaseLock consensus library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    crate2nix = {
      url = "github:balsoft/crate2nix/balsoft/fix-broken-ifd";
      flake = false;
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zig.url = "github:arqv/zig-overlay";
  };

  outputs = { self, nixpkgs, flake-compat, utils, crate2nix, fenix, zig}:
    utils.lib.eachDefaultSystem (system:
      let
        macos_sdk = builtins.fetchTarball {
          url = "https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz";
          sha256 = "sha256-BoFWhRSHaD0j3dzDOFtGJ6DiRrdzMJhkjxztxCluFKo=";
        };
        fenixStable = fenix.packages.${system}.stable.withComponents [ "cargo" "clippy" "rust-src" "rustc" "rustfmt" "llvm-tools-preview" ];
        # needed for compiling static binary
        fenixMusl = with fenix.packages.${system}; combine [ (stable.withComponents [ "cargo" "clippy" "rustc" "rustfmt" ]) targets.x86_64-unknown-linux-musl.stable.rust-std ];

        rustOverlay = final: prev:
          {
            rustc = fenixStable;
            cargo = fenixStable;
            rust-src = fenixStable;
          };

        pkgs = import nixpkgs {
          config.allowBroken = true;
          inherit system;
          overlays = [
            rustOverlay
          ];
        };

        cargo-llvm-cov = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-llvm-cov";
          version = "0.3.0";

          doCheck = false;

          buildInputs = [ pkgs.libllvm ];

          src = builtins.fetchTarball {
            url = "https://crates.io/api/v1/crates/${pname}/${version}/download";
            sha256 = "sha256:0iswa2cdaf2123vfc42yj9l8jx53k5jm2y51d4xqc1672hi4620l";
          };

          cargoSha256 = "sha256-RzIkW/eytU8ZdZ18x0sGriJ2xvjVW+8hB85In12dXMg=";
          meta = with pkgs.lib; {
            description = "Cargo llvm cov generates code coverage via llvm.";
            homepage = "https://github.com/taiki-e/cargo-llvm-cov";

            license = with licenses; [mit asl20 ];
          };
        };


        # DON'T FORGET TO PUT YOUR PACKAGE NAME HERE, REMOVING `throw`
        crateName = "phaselock";

        inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
          generatedCargoNix;

        project = import
          (generatedCargoNix {
            name = crateName;
            src = ./.;
          })
          {
            inherit pkgs;
            # need to force threshold_crypto/use-insuecure-test-only-mock-crypto
            # otherwise a subset of tests hang
            rootFeatures = [ "demo" "docs" "blake3" "threshold_crypto/use-insecure-test-only-mock-crypto" ];
            defaultCrateOverrides = pkgs.defaultCrateOverrides // {
              # Crate dependency overrides go here
              # pass in protobuf
              prost-build = attrs: {
                buildInputs = [ pkgs.protobuf ];
                PROTOC = "${pkgs.protobuf}/bin/protoc";
                PROTOC_INCLUDE = "${pkgs.protobuf}/include";
              };
              libp2p-core = attrs: {
                buildInputs = [ pkgs.protobuf ];
                PROTOC = "${pkgs.protobuf}/bin/protoc";
                PROTOC_INCLUDE = "${pkgs.protobuf}/include";
              };
            };
          };
          zig_cc =
# -F${pkgs.darwin.apple_sdk.frameworks.Security}/Library/Frameworks -F${pkgs.darwin.apple_sdk.frameworks.SystemConfiguration}/Library/Frameworks -F${pkgs.darwin.apple_sdk.frameworks.System}/Library/Frameworks -F${pkgs.darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks
            pkgs.writeScriptBin "zig_cc" ''
              #!/usr/bin/env bash
              ${zig.packages.aarch64-darwin."0.9.1"}/bin/zig cc -target aarch64-macos.11 -L${pkgs.libiconv}/lib -F${macos_sdk}/System/Library/Frameworks -Wl,-undefined=dynamic_lookup $@
            '';



        # TODO uncomment when fetching dependencies is unborked
        # pkgsAndChecksList = pkgs.lib.mapAttrsToList (name: val: { packages.${name} = val.build; checks.${name} = val.build.override { runTests = true; }; }) project.workspaceMembers;
        # # programatically generate output packages based on what exists in the workspace
        # pkgsAndChecksAttrSet = pkgs.lib.foldAttrs (n: a: pkgs.lib.recursiveUpdate n a) { } pkgsAndChecksList;

        buildDeps = with pkgs; [
          cargo-audit
          nixpkgs-fmt
          git-chglog
          protobuf
          python3
          zlib.dev
          zlib.out
          fenix.packages.${system}.rust-analyzer

        ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security pkgs.libiconv darwin.apple_sdk.frameworks.SystemConfiguration darwin.apple_sdk.frameworks.System];

      in
      {
        devShell = pkgs.mkShell {
          #ZIG_GLOBAL_CACHE_DIR = "";
          CC = "${zig_cc}/bin/zig_cc";
          RUSTFLAGS = "-Clinker=${zig_cc}/bin/zig_cc ";
          shellHook = ''
                export ZIG_GLOBAL_CACHE_DIR=$(mktemp -d)
          '';
          buildInputs =
            with pkgs; [ fenixStable zld zig_cc] ++ buildDeps;
        };


        devShells = {
          zigldShell = pkgs.mkShell {
            CC = "${zig_cc}/bin/zig_cc";
            RUSTFLAGS = "-Clinker=${zig_cc}/bin/zig_cc ";
            shellHook = ''
                  export ZIG_GLOBAL_CACHE_DIR=$(mktemp -d)
            '';
            buildInputs =
              with pkgs; [ fenixStable zig_cc] ++ buildDeps;
          };
          zldShell = pkgs.mkShell {
            RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.zld}/bin/zld";
            buildInputs =
              with pkgs; [ fenixStable zld] ++ buildDeps;
          };




          # usage: compile a statically linked musl binary
          staticShell = pkgs.mkShell {
            shellHook = ''
              ulimit -n 1024
              export RUSTFLAGS='-C target-feature=+crt-static'
              export CARGO_BUILD_TARGET='x86_64-unknown-linux-musl'
            '';
            buildInputs =
              with pkgs; [ fenixMusl ] ++ buildDeps;
          };

          # usage: evaluate performance (grcov + flamegraph)
          perfShell = pkgs.mkShell {
            buildInputs = with pkgs; [ flamegraph fd cargo-llvm-cov fenixStable zld sccache] ++ buildDeps;
          };

        };
        # TODO uncomment when fetching dependencies is unborked
        # packages = pkgsAndChecksAttrSet.packages;
        # checks = pkgsAndChecksAttrSet.checks;

        defaultPackage = project.workspaceMembers.phaselock.build;
      }
    );
}
