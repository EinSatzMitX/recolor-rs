{
  lib,
  stdenv,
  fetchFromGitHub,
  pkg-config,
  rust-analyzer,
  rustfmt,
  clippy,
}:
stdenv.mkDerivation (finalAttrs: {
  pname = "recolor-rs";
  version = "0.1.0";

  # src = fetchFromGitHub {
  #   owner = "EinSatzMitX";
  #   repo = "recolor-rs";
  #   rev = "v0.1.0";
  #   sha256 = "0aj9k5g837njl4d9pdjb6aidiqma8bxbggz5qfcjvwz9zpiwvzix";
  # };

  src = ./.;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    rust-analyzer # LSP Server
    rustfmt # Formatter
    clippy # Linter
  ];

  strictDeps = true;
  enableParallelBuilding = true;

  # postPatch = ''
  #   substituteInPlace CMakeLists.txt \
  #     --replace-fail " -march=native" "" \
  #     --replace-fail " -mtune=native" "" \
  #     --replace-fail "-Wl,-s" "" \
  #     --replace-fail " -s" "" \
  #     --replace-fail 'set(CMAKE_INSTALL_PREFIX "/usr" CACHE PATH "Installation prefix" FORCE)' ""
  # '';

  # cmakeFlags = [
  #   "-G Ninja"
  #   "-DCMAKE_CXX_COMPILER=clang++"
  #   "-DCMAKE_C_COMPILER=clang"
  #   "-DCMAKE_BUILD_TYPE=Release"
  # ];

  meta = {
    description = "Recoloring tool for images";
    homepage = "https://github.com/EinSatzMitX/recolor-rs";
    license = lib.licenses.gpl3;
    maintainers = with lib.maintainers; [EinSatzMitX];
    platforms = lib.platforms.x86_64;
    # platforms = with lib.platforms; [x86_64-linux];

    # badPlatforms = lib.platforms.darwin;
  };
})
