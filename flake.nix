{
  description = "Testing";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
  };
  outputs = {self, nixpkgs, ...}:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.leagacypkgs.${system};
    in
    {
      devShells.${system}.default = (import ./shell.nix {inherit pkgs;});
    };
}
