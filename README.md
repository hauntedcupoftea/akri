# Akri

Akri (Greek: _Akribeia_, meaning Precision) is a basic tracking dashboard for
test scores. It was built to track progress in competitive exams (JEE, BITSAT,
etc.) with support for custom marking schemes, negative marking, and
subject-wise analytics.

**Tech Stack:** Tauri v2 (Rust), Svelte 5, SQLite, Chart.js.

![Akri Screenshot](https://via.placeholder.com/800x450.png?text=Akri+Dashboard+Screenshot)

## Features

- **Flexible Marking Schemes:** Native support for negative marking (e.g.,
  +4/-1) or flat scoring.
- **Templates:** Create presets for different exams (JEE Mains, CET, Boards) to
  auto-fill subjects and rules.
- **Historical Integrity:** Changing a template does not break historical data.
  Every test saves a snapshot of the rules used at that time.
- **Performance Graph:** Visual trajectory of Score % vs Accuracy %.
- **Filterable:** Instantly toggle the graph between different exam formats.
- **Privacy Focused:** 100% local SQLite database. No cloud, no tracking.

## Installation

### Nix/OS

You can install it declaratively on NixOS or via Home Manager without manually
compiling.

#### 1. Quick Try

Run immediately without installing:

```bash
nix run github:hauntedcupoftea/akri
```

#### 2. Add to Configuration (NixOS / Home Manager)

Add Akri to your `flake.nix` inputs:

```Nix
{
  inputs.akri.url = "github:hauntedcupoftea/akri";
  # Optional: Deduplicate dependencies
  inputs.akri.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, akri, ... }: {
    # For NixOS:
    nixosConfigurations.hostname = nixpkgs.lib.nixosSystem {
      modules = [
        ({ pkgs, ... }: {
          environment.systemPackages = [ akri.packages.${pkgs.system}.default ];
        })
      ];
    };

    # For Home Manager:
    homeConfigurations."username" = home-manager.lib.homeManagerConfiguration {
      modules = [
        ({ pkgs, ... }: {
          home.packages = [ akri.packages.${pkgs.system}.default ];
        })
      ];
    };
  };
}
```

### Other

See the Releases tab for precompiled binaries of newer versions.

## 🛠️ Development Setup

You need **Rust** and **Deno** installed.

1. **Clone the repo**
   ```bash
   git clone https://github.com/hauntecupoftea/akri.git
   cd akri
   ```

2. **Install Dependencies**
   ```bash
   deno install
   ```

3. **Run in Development Mode**
   ```bash
   deno task tauri dev
   ```

## 📦 Building for Release

To create a standalone executable (`.exe`, `.dmg`, or `.deb`):

```bash
deno task tauri build
```

The output binaries will be located in [[./src-tauri/target/release/]]. 🗄️

## Database Location

Akri uses a local SQLite database named tracking.db.

    Linux: ~/.local/share/com.hauntedcupoftea.akri/tracking.db

    Windows: %APPDATA%\com.hauntedcupofteatea.akri\tracking.db

    macOS: ~/Library/Application Support/com.hauntedcupoftea.akri/tracking.db

## License

MIT. Built for a one-day fun side project challenge.

## Disclaimer

- Logo was created by ChatGPT. If enough people like the project and want me to
  change it maybe I will.
- Some code was written with the help of AI (Gemini 3 Pro Preview), mainly for
  the workflow, documentation and some css help.
