{
  description = "Learn Rust";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

      in with pkgs; {
        devShell = mkShell {
          nativeBuildInputs = [ autoPatchelfHook pkg-config ];
          buildInputs =
            [ rust-analyzer cargo-edit rustup openssl openssl.dev systemd ];
        };
      });
}
