{
  description = "Testing";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
  };
  outputs = {self, nixpkgs, lib, ...}:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = (import ./shell.nix {inherit pkgs;});
    };
}
