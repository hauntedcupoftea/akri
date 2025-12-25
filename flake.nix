{
  description = "Akri - Precision Test Tracker";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };

        libraries = with pkgs; [
          webkitgtk_4_1
          gtk3
          cairo
          gdk-pixbuf
          glib
          glib-networking
          libsoup_3
          pango
          harfbuzz
          at-spi2-atk
          openssl
          sqlite
          librsvg
        ];

        pname = "akri";
        version = "0.1.0";
        description = "Local-first test tracking dashboard";

        frontend = pkgs.stdenv.mkDerivation {
          name = "${pname}-frontend-${version}";
          src = ./.;
          nativeBuildInputs = [pkgs.deno];
          outputHashAlgo = "sha256";
          outputHashMode = "recursive";
          outputHash = "sha256-B5KWHUiwm4x1HsBDmy/3r5cFn2XaEmWOEk4dGlpi8Mo=";

          buildPhase = ''
            export DENO_DIR=$TMPDIR/deno-cache
            deno install --allow-scripts
            export PATH=$PWD/node_modules/.bin:$PATH
            deno task build
          '';

          installPhase = ''
            mkdir -p $out
            cp -r build/* $out/
          '';
        };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit pname version;
          src = ./.;

          cargoLock = {
            lockFile = ./src-tauri/Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          cargoRoot = "src-tauri";
          buildAndTestSubdir = "src-tauri";

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
            cargo-tauri.hook
            wrapGAppsHook4
            copyDesktopItems
          ];

          buildInputs = libraries;

          postPatch = ''
            mkdir -p build
            cp -r ${frontend}/* build/

            substituteInPlace src-tauri/tauri.conf.json \
              --replace '"beforeBuildCommand": "deno task build",' '"beforeBuildCommand": "",'
          '';

          desktopItems = [
            (pkgs.makeDesktopItem {
              name = pname;
              desktopName = "Akri";
              comment = description;
              exec = "akri";
              icon = pname;
              categories = ["Education" "Utility"];
            })
          ];

          postInstall = ''
            if [ -f src-tauri/icons/128x128.png ]; then
              install -Dm644 src-tauri/icons/128x128.png $out/share/icons/hicolor/128x128/apps/${pname}.png
            fi

            wrapProgram $out/bin/akri \
              --set LD_LIBRARY_PATH "${pkgs.lib.makeLibraryPath libraries}" \
              --set WEBKIT_DISABLE_DMABUF_RENDERER "1"
          '';

          meta = {
            inherit description;
            homepage = "https://github.com/tea/akri";
            license = pkgs.lib.licenses.mit;
            mainProgram = "akri";
          };
        };

        devShells.default = pkgs.mkShell {
          name = "akri-dev";
          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
            cargo-tauri
            deno
            wrapGAppsHook4
          ];
          buildInputs = libraries;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
          WEBKIT_DISABLE_DMABUF_RENDERER = "1";
          shellHook = ''
            echo "Rust Version: $(rustc --version)"
            echo "Deno Version: $(deno --version)"
            echo "Tauri Environment Ready."

            if [[ $- == *i* ]]; then
              exec fish
            fi
          '';
        };
      };
    };
}
