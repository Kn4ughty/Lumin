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
        lumin = pkgs.callPackage ./package.nix { };
      in
      {
        packages = {
          inherit lumin;
          default = lumin;
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
          RUSTFLAGS = "-C link-arg=-Wl,-rpath,${lumin.libPath}";
        };
      }
    )
    // {
      overlays.default = final: prev: {
        lumin = final.callPackage ./package.nix { };
      };
    };
}
