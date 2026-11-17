{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  outputs =
    { self, nixpkgs }:
    let
      systems = nixpkgs.lib.systems.flakeExposed;
      forAllSystem = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              clippy
            ];
          };
        }
      );
    };
}
