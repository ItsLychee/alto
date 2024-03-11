{
  description = "A metadata-based media organizer";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      inherit (builtins) attrValues;
      eachSystem = f:
        lib.genAttrs [ "x86_64-linux" "aarch64-linux" ]
        (system: f nixpkgs.legacyPackages.${system});
    in {

      packages = eachSystem (pkgs: rec {
        default = alto;
        alto = pkgs.callPackage ./package.nix { };
      });

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = attrValues { inherit (pkgs) cargo rustc rust-analyzer; };
        };
      });

      formatter = eachSystem (pkgs: pkgs.nixfmt);

    };
}
