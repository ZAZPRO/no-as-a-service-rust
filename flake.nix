{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
          f = with fenix.packages.${system}; combine [
            stable.toolchain
          ];
      in {
        nixpkgs.overlays = [fenix.overlays.default];
        devShells.default = pkgs.mkShell {
          packages = [
            f
            pkgs.rust-analyzer
          ];
        };
      }
    );
}
