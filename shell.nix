{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell rec {
    name = "shell";
    nativeBuildInputs = [pkgconfig protobuf cmake gdb ];
    buildInputs = [
        openssl freetype expat
        vulkan-loader vulkan-tools
        wayland wayland-protocols libxkbcommon swiftshader
    ] ++ (with xorg; [
        libX11 libXcursor libXrandr libXi
    ]);
    shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath buildInputs}";
    '';
}
