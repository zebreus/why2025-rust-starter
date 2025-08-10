{
  description = "A rust project for the WHY2025 badge";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
          ];
        };

        fenixPkgs = fenix.packages.${system};
        rustToolchain = fenixPkgs.combine [
          (fenixPkgs.targets.riscv32imafc-unknown-none-elf.latest.withComponents [
            "rust-std"
          ])
          (fenixPkgs.latest.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
        ];
      in
      {
        name = "why2025-rust-starter";

        devShell = pkgs.mkShell {
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer-nightly
          ];
        };

        # TODO: Crosscompile for riscv32imafc-unknown-none-elf with nix is difficult.
        # packages.why2025-rust-starter =
        #   (pkgs.makeRustPlatform {
        #     cargo = rustToolchain;
        #     rustc = rustToolchain;
        #   }).buildRustPackage
        #     {
        #       pname = "why2025-rust-starter";
        #       version = "0.1.0";
        #       src = ./.;
        #       cargoLock = {
        #         lockFile = ./Cargo.lock;
        #       };
        #       target = "x86_64-unknown-linux-muslsdfasdf";
        #     };
        # packages.default = packages.why2025-rust-starter;

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
