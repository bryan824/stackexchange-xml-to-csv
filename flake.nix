{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-21.11";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          pkgs = nixpkgs.legacyPackages.${system};
          target = "x86_64-unknown-linux-gnu";
          toolchain = with fenix.packages.${system}; combine [
            minimal.cargo
            minimal.rustc
            targets.${target}.latest.rust-std
          ];
        in

        (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          buildInputs = [ pkgs.patchelf pkgs.gcc10 ];
          CARGO_BUILD_TARGET = target;
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER =
            "${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/${target}-gcc";
        };
    });
}
