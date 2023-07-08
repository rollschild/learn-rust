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
        rustVersion = pkgs.rust-bin.stable.latest.default;

      in with pkgs; {
        devShell = mkShell {
          nativeBuildInputs = [ autoPatchelfHook pkg-config ];
          buildInputs = [
            (rustVersion.override { extensions = [ "rust-src" ]; })
            rust-analyzer
            rustup
            clippy
            evcxr
            openssl
            openssl.dev
            systemd
          ];
        };
      });
}
