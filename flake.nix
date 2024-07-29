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
            version = "1.0.0-dev00";
            src = ./.;

            cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };

            buildInputs = with pkgs; [
              openssl
              sqlite
            ];

            nativeBuildInputs = with pkgs; [
              # rust deps
              cairo
              clippy
              rust-analyzer
              rustc
              rustfmt

              # glib deps
              gtk4
              libadwaita
              wrapGAppsHook4

              # compiler deps
              appstream
              appstream-glib
              desktop-file-utils
              glib
              libxml2
              meson
              ninja
              pkg-config

              # library deps
              fish
              openssl
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

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # rust deps
            cairo
            clippy
            rust-analyzer
            rustc
            rustfmt

            # glib deps
            gtk4
            libadwaita

            # compilation deps
            appstream
            appstream-glib
            desktop-file-utils
            glib
            libxml2
            meson
            ninja
            pkg-config

            # library deps
            fish
            openssl
            sqlite
          ];

          shellHook = "exec fish";
        };
      }
    );
}
