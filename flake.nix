{
  description = "A Nix Flake for a Rust API using Axum, Diesel, and Tokio";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Use the latest stable Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # System dependencies needed for building/running the project
        runtimeDeps = with pkgs; [
          openssl
          libiconv
          postgresql # Provides libpq required by Diesel
        ];

        buildDeps = with pkgs; [
          pkg-config
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = runtimeDeps ++ buildDeps ++ [
            rustToolchain
            pkgs.diesel-cli  # Installed with postgres support by default in nixpkgs
          ];

          # Make sure Diesel can find the postgres libraries at compile/run time
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeDeps}:$LD_LIBRARY_PATH"
            export DATABASE_URL="postgres://postgres:password@localhost:5432/rust_api_test"

            # Universal aliases that work flawlessly across Fish, Bash, and Zsh
            alias db-start="docker compose up -d"
            alias db-stop="docker compose down"

            echo "Rust & Diesel Dev Environment Loaded!"
          '';
        };
      }
    );
}