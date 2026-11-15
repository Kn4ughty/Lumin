{
  lib,
  rustPlatform,
  pkg-config,
  patchelf,
  wayland,
  libxkbcommon,
  libGL,
  vulkan-loader,
  aws-lc,
}:
let
  runtimeLibs = [
    wayland
    libxkbcommon
    libGL
    vulkan-loader
    aws-lc
  ];
  libPath = lib.makeLibraryPath runtimeLibs;
in
rustPlatform.buildRustPackage {
  pname = "lumin";
  version = "1.1.0";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };
  nativeBuildInputs = [
    pkg-config
    patchelf
  ];
  buildInputs = [ aws-lc ];
  postFixup = ''
    patchelf --add-rpath ${libPath} $out/bin/lumin
  '';
  passthru = { inherit libPath; };
}
