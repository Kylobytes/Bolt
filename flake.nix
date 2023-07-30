{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
            inherit system;
        };
      in {
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

            # compiler deps
            appstream-glib
            desktop-file-utils
            glib
            libxml2
            meson
            ninja
            pkg-config
          ];
        };
      }
    );
}
