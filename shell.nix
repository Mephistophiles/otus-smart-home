{ pkgs ? import <nixpkgs> {} }:
with pkgs;
pkgs.mkShell {
  nativeBuildInputs = [ pkg-config python3 protobuf rustfmt cmake ];
  buildInputs = [ openssl dbus zlib libgit2 xorg.libxcb ];
  PROTOC = "${protobuf}/bin/protoc";
  shellHook = ''export CFG_DISABLE_CROSS_TESTS=1'';
}
