{
  description = "Dory - A fast, keyboard-first database client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      flake-utils,
      ...
    }:
    let
      releaseInfo = import ./nix/release-info.nix;

      # Systems that ship a prebuilt binary in the matching GitHub Release.
      # Other systems can still use the source build.
      prebuiltSystems = builtins.attrNames releaseInfo.artifacts;

      # Per-system outputs (packages, devShells, apps).
      perSystem = flake-utils.lib.eachDefaultSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          # OpenSSL built with static libraries for portable binaries.
          # The default nixpkgs openssl only ships shared objects; this override
          # enables the static output so OPENSSL_STATIC=1 works at build time.
          opensslStatic = pkgs.openssl.override { static = true; };

          # Import default.nix with crane support
          dory = import ./default.nix {
            inherit pkgs craneLib;
            version = "0.1.0";
          };

          # Source build (current behavior, compiles locally via crane).
          dorySource = dory.buildWithCrane craneLib;

          # Prebuilt-binary build, only when an artifact exists for this system.
          hasPrebuilt = builtins.elem system prebuiltSystems;
          doryBin =
            if hasPrebuilt then
              pkgs.callPackage ./nix/binary.nix { }
            else
              null;

          # Rolling nightly prebuilt — pinned on the `nightly` ref.
          # On `main` the hashes in nightly-info.nix are placeholders; always
          # consume this via `github:vbasky/dory/nightly#dory-nightly`.
          doryNightly =
            if hasPrebuilt then
              pkgs.callPackage ./nix/binary.nix { infoFile = ./nix/nightly-info.nix; }
            else
              null;

          # Default package: prefer the prebuilt binary when available
          # (fast install for end users), fall back to the source build.
          doryDefault = if hasPrebuilt then doryBin else dorySource;
        in
        {
          # Development shell
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = dory.nativeBuildInputs ++ [
              rustToolchain
              pkgs.rust-analyzer
              opensslStatic.dev
              # Faster, process-isolated test runner for the large workspace.
              # Run with `cargo nextest run` (doctests still need `cargo test --doc`).
              # mold is inherited from dory.nativeBuildInputs (see default.nix).
              pkgs.cargo-nextest
              # Detects unused dependency declarations (DEP-2 regression guard).
              pkgs.cargo-machete
            ];

            buildInputs = dory.buildInputs;

            LD_LIBRARY_PATH = dory.runtimeLibraryPath;
            ZSTD_SYS_USE_PKG_CONFIG = "1";

            # Link OpenSSL statically so the binary runs outside the Nix store
            # (e.g. on Arch Linux without /nix/store available at runtime).
            OPENSSL_STATIC = "1";
            OPENSSL_LIB_DIR = "${opensslStatic.out}/lib";
            OPENSSL_INCLUDE_DIR = "${opensslStatic.dev}/include";

            shellHook = ''
              echo "Dory development environment loaded (Nix flake)"
              echo "Run 'cargo build' to build the project"
              echo "Run 'nix build' to build the default package"
              echo "Run 'nix flake check' to run all checks"
            '';
          };

          # Packages:
          #   .default         -> prebuilt when available, source otherwise
          #   .dory          -> alias for .default
          #   .dory-bin      -> explicit prebuilt (only on supported systems)
          #   .dory-source   -> explicit source build
          #   .dory-nightly  -> rolling nightly prebuilt (pin to nightly ref)
          packages = {
            default = doryDefault;
            dory = doryDefault;
            dory-source = dorySource;
          } // (if hasPrebuilt then {
            dory-bin = doryBin;
            dory-nightly = doryNightly;
          } else { });

          formatter = pkgs.nixpkgs-fmt;

          # Apps
          apps = {
            default = flake-utils.lib.mkApp {
              drv = doryDefault;
              exePath = "/bin/dory";
            };

            dory = flake-utils.lib.mkApp {
              drv = doryDefault;
              exePath = "/bin/dory";
            };
          } // (if hasPrebuilt then {
            dory-nightly = flake-utils.lib.mkApp {
              drv = doryNightly;
              exePath = "/bin/dory-nightly";
            };
          } else { });
        }
      );
    in
    perSystem // {
      # Overlay for downstream consumers:
      #
      #   nixpkgs.overlays = [ inputs.dory.overlays.default ];
      #   environment.systemPackages = [ pkgs.dory ];
      #
      # `pkgs.dory`         -> prebuilt binary (fast)
      # `pkgs.dory-source`  -> built from source via crane
      # `pkgs.dory-bin`     -> explicit prebuilt (only on prebuilt systems)
      # `pkgs.dory-nightly` -> rolling nightly prebuilt (only on prebuilt systems)
      overlays.default = final: prev:
        let
          system = prev.stdenv.hostPlatform.system;
          hasSystem = perSystem.packages ? ${system};
          sysPkgs = perSystem.packages.${system};
        in
        if hasSystem then
          {
            dory = sysPkgs.dory;
            dory-source = sysPkgs.dory-source;
          }
          // nixpkgs.lib.optionalAttrs (sysPkgs ? dory-nightly) {
            dory-nightly = sysPkgs.dory-nightly;
          }
          // nixpkgs.lib.optionalAttrs (sysPkgs ? dory-bin) {
            dory-bin = sysPkgs.dory-bin;
          }
        else
          { };
    };
}
