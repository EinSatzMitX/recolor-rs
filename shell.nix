{pkgs ? import <nixpkgs> {}}: let
  recolor-rs = pkgs.callPackage ./recolor-rs.nix {};
in
  pkgs.mkShell {
    name = "recolor-rs";

    buildInputs = [recolor-rs];
  }
