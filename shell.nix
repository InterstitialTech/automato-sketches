# shell for arduino dev on nixos
#
# put this in a directory and `nix-shell` to get these tools.
#
# or if you use direnv, make a .envrc containing `use_nix` for automatic loading when you cd
# to the directory.
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = [
      pkgs.python311Packages.pyserial
      pkgs.arduino-cli
      pkgs.screen
    ];
}

