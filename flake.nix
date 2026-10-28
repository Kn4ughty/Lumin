{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        runtimeLibs = with pkgs; [
          wayland
          # required?
          libxkbcommon
          libGL
          vulkan-loader
        ];
        libPath = pkgs.lib.makeLibraryPath runtimeLibs;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "lumin";
          version = "1.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };
          nativeBuildInputs = with pkgs; [
            pkg-config
            patchelf
          ];
          postFixup = ''
            patchelf --add-rpath ${libPath} $out/bin/lumin
          '';
        };
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.pkg-config ];
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          RUSTFLAGS = "-C link-arg=-Wl,-rpath,${libPath}";
        };
      }
    );
}
