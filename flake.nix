{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    dev-shell = pkgs.callPackage ./nix/shell.nix {};
    naas = pkgs.callPackage ./nix/default.nix {};
  in {
    devShells.${system}.default = dev-shell;

    packages.${system}.default = naas;
  };
}
