# Vert

**v0.2.0** — Universal file format converter — desktop app + CLI. Built with Tauri 2 + Vue 3 + Rust.

[中文](README.zh.md) | [Apache 2.0 License](LICENSE)

## Supported formats (21)

| Category | Formats |
|---|---|
| **Config** (6) | `.properties` `.yaml` `.yml` `.json` `.toml` `.xml` `.ini` `.cfg` `.conf` — any to any |
| **Data** (1) | `.csv` ↔ `.json` `.xml` (tabular) |
| **Document** (6) | `.epub` `.docx` `.pdf` `.md` `.markdown` `.html` `.htm` `.txt` `.text` |
| **Image** (7) | `.svg` `.png` `.jpg` `.jpeg` `.gif` `.webp` `.bmp` `.tiff` — any to any |

**Image conversions**: SVG rasterizes to any image format via resvg; PNG/JPEG/GIF/WebP/BMP/TIFF use the pure-Rust `image` crate.

## Install

### Desktop app

Download from [Releases](https://github.com/jean3690/vert/releases).

Or build from source:

```bash
pnpm install
pnpm tauri build
```

### CLI

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/jean3690/vert/master/install.ps1 | iex
```

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/jean3690/vert/master/install.sh | bash
```

Or via cargo:

```bash
cd src-tauri
cargo build --bin vert-cli --no-default-features
./target/debug/vert-cli --help
```

## CLI usage

```bash
# Convert
vert data.json yaml
vert config.xml json -o output.json
vert document.docx pdf
vert logo.svg png

# Explore
vert input.csv              # show valid target formats
vert --list                 # list all supported formats
```

## Develop

```bash
pnpm install                # frontend dependencies

pnpm dev                    # Vite dev server (port 1420)
pnpm tauri dev              # full Tauri desktop app (dev)
pnpm build                  # type-check + production build

cd src-tauri
cargo test                  # Rust unit tests (48+ tests)
cargo build --bin vert-cli --no-default-features  # CLI binary only
```

Requires Liberation Sans fonts in `src-tauri/fonts/` for DOCX/MD/HTML → PDF conversions.

## License

Apache 2.0 — see [LICENSE](LICENSE).
