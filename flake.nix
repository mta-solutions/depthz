{
  description = "rust dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libs = with pkgs; [
        ];
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = with pkgs; [
            ];
            nativeBuildInputs = [
              gcc
              (rust-bin.stable.latest.default.override
                {
                  extensions = ["rust-src" "rust-analyzer"];
                })
              pkg-config
            ];
            shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath libs}"
            '';
          };
        }
    );
}
