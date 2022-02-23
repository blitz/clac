{
  description = "A reverse-polish command line calculator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-21.11";
    naersk.url = "github:nix-community/naersk";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-compat-ci.url = "github:hercules-ci/flake-compat-ci";
  };

  outputs = { self, utils, nixpkgs, naersk, flake-compat, flake-compat-ci }:
    (utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages = {
        clac = naersk-lib.buildPackage {
          pname = "clac";
          root = ./.;
        };
      };

      defaultPackage = packages.clac;

      # `nix run`
      apps.clac = utils.lib.mkApp {
        drv = packages.clac;
      };
      defaultApp = apps.clac;
    })) // {
      # For Hercules CI, which doesn't natively support flakes (yet).
      ciNix = flake-compat-ci.lib.recurseIntoFlakeWith {
        flake = self;

        # Optional. Systems for which to perform CI.
        systems = [ "x86_64-linux" ];
      };
    };
}
