{
  description = "Dominator Unofficial Website";
  # To update all inputs:
  # $ nix flake update --recreate-lock-file
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/master";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  inputs.devshell.url = "github:numtide/devshell/master";
  # Use the same version of nixpkgs as this project.
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, flake-utils, devshell, rust-overlay }:
    {
      overlay = import ./nix/overlay.nix;
    }
    //
    (
      flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            # Makes the config pure as well. See <nixpkgs>/top-level/impure.nix:
            config = {
              allowBroken = true;
              permittedInsecurePackages = [
                "openssl-1.0.2u"
              ];
            };
            overlays = [
              (import rust-overlay)
              devshell.overlay
              self.overlay
            ];
          };
        in
        {
          legacyPackages = pkgs.dominator;

          defaultPackage = pkgs.dominator.nix.rust-frontend;

          packages = flake-utils.lib.flattenTree pkgs.dominator;

          devShell = import ./nix/devshell.nix { inherit pkgs; };

          checks = { };
        }
      )
    );
}
