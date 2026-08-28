{ pkgs ? import <nixpkgs> {} }:

let
  dory = import ./default.nix { inherit pkgs; };
in
dory.shell
