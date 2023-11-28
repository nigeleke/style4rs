{
  description = "style4rs-dev";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/master;
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.rustup
          pkgs.vscode
        ];
      };
    };

}
