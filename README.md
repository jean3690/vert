# Vert

File format converter — desktop app + CLI. Built with Tauri 2 + Vue 3 + Rust.

[中文](README.zh.md)

## Supported formats

| Category | Source → Target |
|---|---|
| **Config** | `.properties` `.yaml` `.json` `.toml` `.xml` — any to any |
| **Data** | `.csv` ↔ `.json` `.xml` (tabular) |
| **Document** | `.docx` → `.pdf` `.html` `.md` &nbsp;|&nbsp; `.md` ↔ `.html` `.pdf` &nbsp;|&nbsp; `.html` → `.pdf` `.md` |

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
cargo build --bin vert
./target/debug/vert --help
```

## CLI usage

```bash
# Convert
vert data.json yaml
vert config.xml json -o output.json
vert document.docx pdf

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
cargo test                  # Rust unit tests
cargo build --bin vert      # CLI binary only
```

Requires Liberation Sans fonts in `src-tauri/fonts/` for DOCX/MD/HTML → PDF conversions.
