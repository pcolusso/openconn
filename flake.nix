
{
  description = "Just a script to prevent endless aws cli invocations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
        rustPackage = pkgs.rustBuilder.makePackage {
          name = "openconn";
          src = ./.;
        };
      in
      {
        packages.default = rustPackage;
        defaultPackage = rustPackage;
        devShell = pkgs.mkShell {
          buildInputs = [ rustPackage ];
        };
      });
}
