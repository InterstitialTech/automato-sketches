# shell for arduino dev on nixos
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = [
      pkgs.python311Packages.pyserial
      pkgs.arduino-cli
      pkgs.screen
    ];
}

