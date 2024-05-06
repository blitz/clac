{
  description = "A reverse-polish command line calculator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, utils, nixpkgs, naersk }:
    (utils.lib.eachSystem [
      "x86_64-linux"
    ] (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";

      clac = naersk-lib.buildPackage {
        pname = "clac";
        root = ./.;
      };
    in rec {
      # `nix build`
      packages = {
        inherit clac;

        default = clac;
      };

      # `nix run`
      apps.clac = utils.lib.mkApp {
        drv = packages.clac;
      };
    }));
}
