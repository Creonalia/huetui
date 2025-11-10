{ lib, pkgs, ... }:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rust-bin.stable.latest.default
    rust-analyzer

    # Needed if you want to use LLD as the linker instead of LD - see the following link
    # https://matklad.github.io/2022/03/14/rpath-or-why-lld-doesnt-work-on-nixos.html
    # llvmPackages.bintools

    # libxkbcommon
    # libGL

    # # WINIT_UNIX_BACKEND=wayland
    # wayland

    # # WINIT_UNIX_BACKEND=x11
    # xorg.libXcursor
    # xorg.libXrandr
    # xorg.libXi
    # xorg.libX11

    # pkg-config
    # libevdev
  ];
  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}
