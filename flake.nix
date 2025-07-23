{
  description = "Material Design in Rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };
  outputs =
    {
      self,
      nixpkgs,
      crane,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f rec {
            pkgs = nixpkgs.legacyPackages.${system};
            lib = pkgs.lib;
            craneLib = crane.mkLib pkgs;
          }
        );
    in
    {
      devShells = forAllSystems (
        { pkgs, craneLib, ... }:
        {
          default = craneLib.devShell {
            packages = with pkgs; [
              pkg-config
              fontconfig
              SDL2
              SDL2_ttf
              SDL2_gfx
            ];
          };
        }
      );
    };
}
