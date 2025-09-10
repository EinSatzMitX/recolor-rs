# shell.nix
{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "recolor-rs-dev";

  # Tools available in your shell
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer
    pkgs.clippy
    pkgs.rustfmt
  ];

  # Optional: set RUST_SRC_PATH for rust-analyzer (helps with std lib lookup)
  RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
}
