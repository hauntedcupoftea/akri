{
  description = "Tauri (Rust) + Svelte + Deno devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {
    flake-parts,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {system, ...}: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer" "clippy"];
        };

        libraries = with pkgs; [
          at-spi2-atk
          atkmm
          cairo
          gdk-pixbuf
          glib
          gtk3
          harfbuzz
          librsvg
          libsoup_3
          pango
          webkitgtk_4_1
          openssl
          sqlite
        ];
      in {
        devShells.default = pkgs.mkShell {
          name = "tauri-deno-dev";

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
            gobject-introspection
            cargo-tauri
            cargo
            deno
          ];

          buildInputs = libraries;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
          XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS";
          OPENSSL_NO_VENDOR = 1;
          SQLITE3_LIB_DIR = "${pkgs.sqlite.out}/lib";

          shellHook = ''
            echo "📦 Rust Version: $(rustc --version)"
            echo "🦕 Deno Version: $(deno --version)"
            echo "🚀 Tauri Environment Ready"

            if [[ $- == *i* ]]; then
              exec fish
            fi
          '';
        };
      };
    };
}
