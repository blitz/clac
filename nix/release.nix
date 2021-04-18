let
  sources = import ./sources.nix;
  pkgs = import sources.nixpkgs {};
  naersk = pkgs.callPackage sources.naersk {};
  
  inherit (import sources."gitignore.nix" { inherit (pkgs) lib; }) gitignoreSource;
in
{
  clac = naersk.buildPackage {
    root = gitignoreSource ../.;
  };
}
