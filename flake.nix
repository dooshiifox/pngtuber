{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust-toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-2wqUa1sFNg8sdadO5YXcUSwpoBek+e+viCiSqnpUbdM=";
        };
      in
      {
        devShells.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            rust-toolchain
            mold
            pkg-config
            alsa-lib
            vulkan-loader
            vulkan-tools
            libudev-zero
            libx11
            libxcursor
            libxi
            libxrandr
            libxkbcommon
            wayland
            kontroll
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold -Zshare-generics=y";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
