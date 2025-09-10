{
  lib,
  fetchFromGitHub,
  rustPlatform,
}: let
  # manifest = lib.importTOML ./Cargo.toml;
in
  rustPlatform.buildRustPackage (finalAttrs: {
    # pname = manifest.name;
    # inherit (manifest) version;
    pname = "recolor-rs";
    version = "0.1.0";

    # For development
    src = ./.;

    # src = fetchFromGitHub {
    #   owner = "EinsatzMitX";
    #   repo = "recolor-rs";
    #   tag = "v${finalAttrs.version}";
    #   hash = "sha256-EeQrz5lF8166sCBjlbgspU1NR7ybjaDFiS4aRGhsD70=";
    # };

    cargoHash = "sha256-kYl0f3QYiMSlSZ2wmolCjMNaEzowIxWDwFAfO9Yx340=";
    # cargoHash = lib.fakeHash;

    meta = {
      description = "A CLI tool to revolor images, written in Rust";
      homepage = "https://github.com/EinSatzMitX/recolor-rs";
      license = lib.licenses.unlicense;
      maintainers = with lib.maintainers; [EinSatzMitX];
    };
  })
