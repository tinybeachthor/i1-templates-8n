{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    with flake-utils.lib;
    eachSystem [ system.x86_64-linux ] (system: let
      pkgs = (import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      });
    in {
      devShells.default = let
        my-rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rls-preview"
            "rust-analyzer-preview"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
          ];
        };
      in pkgs.mkShell {
        buildInputs = with pkgs; [
          git

          my-rust
        ];
      };
    });
}
