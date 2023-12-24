{
    description = "Listen and boost to podcasts";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, utils, ... }: utils.lib.eachDefaultSystem(system:
        let
            pkgs = import nixpkgs { inherit system; };
        in {
            packages = rec {
                bolt = pkgs.stdenv.mkDerivation rec {
                    pname = "bolt";
                    version = "0.1.0";
                    src = ./.;

                    cargoDeps = pkgs.rustPlatform.importCargoLock {
                        lockFile = ./Cargo.lock;
                    };

                    buildInputs = with pkgs; [
                        blueprint-compiler
                        gtk4
                        libadwaita
                        sqlite
                    ];

                    nativeBuildInputs = with pkgs; [
                        cargo
                        rustPlatform.cargoSetupHook
                        rustc

                        appstream-glib
                        desktop-file-utils
                        gtk4
                        libadwaita
                        meson
                        ninja
                        openssl
                        pkg-config
                        sqlite
                        wrapGAppsHook4
                    ];

                    meta = with pkgs.lib; {
                        description = "Listen to and boost to podcasts";
                        homepage = "https://github.com/kylobytes/bolt";
                        license = with licenses; [
                            gpl3
                        ];
                    };
                };
                default = bolt;
            };

            devShells.default = pkgs.mkShell {
                packages = with pkgs; [
                    # rust deps
                    cargo
                    clippy
                    rust-analyzer
                    rustc
                    rustfmt

                    # glib/gtk deps
                    blueprint-compiler
                    cairo
                    gtk4
                    libadwaita

                    # other deps
                    appstream-glib
                    desktop-file-utils
                    gcc
                    glib
                    libxml2
                    meson
                    ninja
                    openssl
                    pkg-config
                    sqlite
                    wrapGAppsHook4
                ];
            };
        });
}
