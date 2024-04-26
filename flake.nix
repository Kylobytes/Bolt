{
  description = "Listen and boost to podcasts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages = rec {
          bolt = pkgs.stdenv.mkDerivation {
            pname = "bolt";
            version = "0.1.0";
            src = ./.;

            cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };

            buildInputs = with pkgs; [
              openssl
              sqlite
            ];

            nativeBuildInputs = with pkgs; [
              # rust deps
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt

              expat
              freetype
              freetype.dev
              libGL
              wayland
              wayland.dev
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr

              fish
              gcc
              openssl
              pkg-config
              sqlite
            ];

            meta = with pkgs.lib; {
              description = ''
                A podcast fetcher for the GNU/Linux Desktop
                 (and mobile?) with support for discovery through
                 podcastindex.org'';
              homepage = "https://github.com/kylobytes/bolt";
              license = licenses.gpl3;
            };
          };
          default = bolt;
        };

        devShells.default = import ./shell.nix { inherit pkgs; };
      }
    );
}
