# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Vert** — a Tauri v2 desktop file format converter. Vue 3 + TypeScript frontend, Rust backend. Package manager: pnpm.

## Commands

```bash
# Install deps
pnpm install

# Frontend dev server only (port 1420)
pnpm dev

# Full Tauri desktop app (dev)
pnpm tauri dev

# Frontend type-check + build
pnpm build

# Rust tests
cd src-tauri && cargo test

# Production build
pnpm tauri build
```

## Architecture

```
src/                              # Vue 3 frontend
  types/index.ts                  # FileFormat, ConvertResult, QueuedFile types
  utils/formats.ts                # Format detection, labels, valid conversion matrix
  styles/main.css                 # Global styles (light/dark via prefers-color-scheme)
  components/
    ConversionPanel.vue           # Main orchestrator — state, invoke, flow control
    FileDropZone.vue              # Tauri onDragDropEvent + file browse via dialog plugin
    FormatSelector.vue            # Source/target dropdowns, filtered by valid targets
    ConversionResult.vue          # idle/converting/success/error states + open file
  App.vue                         # App shell
  main.ts                         # Entry: createApp + global CSS

src-tauri/                        # Rust/Tauri backend
  src/
    error.rs                      # ConversionError enum (manual Display/Error impls)
    commands.rs                   # Tauri commands: convert_file, get_valid_targets
    converter/
      mod.rs                      # Dispatch: routes by Format::category() → domain module
      traits.rs                   # Format enum (from_str, extension, valid_targets, category)
      config.rs                   # Properties/YAML/JSON/TOML/XML ↔ serde_json::Value
      data.rs                     # CSV/JSON/XML ↔ Vec<HashMap<String,String>>
      document.rs                 # DOCX→PDF (quick-xml + zip + genpdf), MD→HTML (pulldown-cmark)
    lib.rs                        # Module registration + plugin/command setup
    main.rs                       # Binary entry
  fonts/                          # PDF fonts (see #fonts below)
  tauri.conf.json                 # Window config (900×700), bundle, build hooks
  capabilities/default.json       # Permissions: core:default, opener:default, dialog:default
```

**Data flow**: User drops file → Tauri `onDragDropEvent` captures native path → frontend detects source format → user picks target format from filtered list → `invoke("convert_file", {filePath, sourceFormat, targetFormat})` → Rust reads file, dispatches to converter module, writes output to same directory → returns `{outputPath, outputSize}` → frontend displays result with "Open file" button.

**Category-based dispatch**: Formats are grouped into three categories — `config`, `data`, `document`. The converter dispatches by matching `source.category()` == `target.category()`. Cross-category conversions (e.g., JSON→PDF) are not supported. This is enforced in both Rust (`converter/mod.rs`) and the frontend (`VALID_CONVERSIONS`).

**Config conversion strategy**: All config formats (Properties/YAML/JSON/TOML/XML) convert via `serde_json::Value` as intermediate — O(n+m) instead of O(n×m) direct converters.

**Data conversion strategy**: CSV/JSON/XML table data converts via `Vec<HashMap<String,String>>` (rows of key-value pairs) as the intermediate representation.

**DOCX→PDF**: Pure Rust. `zip` unpacks the DOCX, `quick-xml` parses `word/document.xml` extracting text runs with formatting (bold/italic/headings), `genpdf` renders to PDF. Handles text-focused documents; complex tables and images are simplified.

**Tauri plugins**: `tauri-plugin-opener` — opens output files in the OS default app (`openPath`). `tauri-plugin-dialog` — native file browse dialog for selecting input files.

## Important constraints

- **`src/utils/formats.ts` VALID_CONVERSIONS must stay in sync with Rust `Format::valid_targets()` in `src-tauri/src/converter/traits.rs`.** These are two independent sources of truth for which conversions are allowed. When adding a new conversion path, update both.

## Keyboard shortcuts

- **Enter** — trigger conversion (when source and target are selected)
- **Escape** — reset all state and clear the current file

## Fonts

DOCX→PDF conversion requires Liberation Sans fonts in `src-tauri/fonts/`:
- LiberationSans-Regular.ttf
- LiberationSans-Bold.ttf
- LiberationSans-Italic.ttf
- LiberationSans-BoldItalic.ttf

Download from https://github.com/liberationfonts/liberation-fonts. Without fonts, DOCX→PDF returns a font error. The font loader searches multiple paths including the working directory and `src-tauri/fonts/`.
