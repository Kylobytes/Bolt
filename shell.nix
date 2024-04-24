{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
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

  LD_LIBRARY_PATH =
    builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

  shellHook = "exec fish";
}
