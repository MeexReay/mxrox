{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs =
    with pkgs; [
        pkgsCross.i686-embedded.buildPackages.gcc
        rustup
        gnumake
        grub2
        libisoburn
        qemu
    ];
}