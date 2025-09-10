{
  lib,
  fetchFromGitHub,
  rustPlatform,
}: let
  manifest = lib.importTOML ./Cargo.toml;
in
  rustPlatform.buildRustPackage (finalAttrs: {
    # pname = manifest.name;
    # inherit (manifest) version;
    pname = "recolor-rs";
    version = "0.1.0";

    # For development
    # src = ./.;

    # src = fetchFromGitHub {
    #   owner = "EinsatzMitX";
    #   repo = "recolor-rs";
    #   tag = finalAttrs.version;
    #   # hash = lib.fakeHash;
    #   # hash = "";
    # };

    cargoHash = "sha256-srV+mGolUrG17xNHy5saK3iXLQBdMOm1KoJCt1MvFY8=";
    # cargoHash = lib.fakeHash;

    meta = {
      description = "Fast line-oriented regex search tool, similar to ag and ack";
      homepage = "https://github.com/EinSatzMitX/recolor-rs";
      license = lib.licenses.unlicense;
      maintainers = [];
    };
  })
