# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Vert** — a Tauri v2 desktop file format converter. Vue 3 + TypeScript frontend (UnoCSS + presetWind), Rust backend. Package manager: pnpm.

## Commands

```bash
# Install deps
pnpm install

# Frontend dev server only (port 1420)
pnpm dev

# Full Tauri desktop app (dev)
pnpm tauri dev

# Frontend type-check (vue-tsc --noEmit) + Vite build
pnpm build

# CLI binary (no GUI deps, avoids GTK/WebKit)
cd src-tauri && cargo build --bin vert-cli --no-default-features

# Rust-only checks (run from src-tauri/)
cargo check              # fast compilation check
cargo clippy             # lint
cargo test               # run all tests

# Production build
pnpm tauri build
```

## Architecture

```
src/                              # Vue 3 frontend
  types/index.ts                  # FileFormat, ConvertResult, QueuedFile types
  utils/formats.ts                # Format detection, labels, valid conversion matrix
  i18n/index.ts                   # useI18n() composable — locale ref, t() with {param} interpolation
  i18n/messages.ts                # en/zh string tables
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
    cli.rs                        # CLI binary (no GUI deps) — help, list, convert, show targets
    error.rs                      # ConversionError enum (thiserror derive) — also used as Tauri command error
    commands.rs                   # Tauri commands: convert_file, get_valid_targets (behind #[cfg(feature = "gui")])
    converter/
      mod.rs                      # Dispatch: routes by Format::category() → domain module
      traits.rs                   # Format enum (from_str, extension, valid_targets, category)
      config.rs                   # Properties/YAML/JSON/TOML/XML ↔ serde_json::Value
      data.rs                     # CSV ↔ JSON/XML via Vec<HashMap<String,String>>
      document.rs                 # DOCX→PDF/HTML/MD (quick-xml+zip+genpdf), MD↔HTML↔PDF (pulldown-cmark, scraper)
    lib.rs                        # Library crate (vert_lib). run() gated behind #[cfg(feature = "gui")]
    main.rs                       # GUI binary entry — just calls vert_lib::run()
  fonts/                          # PDF fonts (see #fonts below)
  tauri.conf.json                 # Window config (900×700), bundle, build hooks
  capabilities/default.json       # Permissions: core:default, opener:default, dialog:default
```

**Data flow**: User drops file → Tauri `onDragDropEvent` captures native path → frontend detects source format → user picks target format from filtered list → `invoke("convert_file", {filePath, sourceFormat, targetFormat})` → Rust reads file, dispatches to converter module, writes output to same directory → returns `{outputPath, outputSize}` → frontend displays result with "Open file" button.

**Category-based dispatch**: Formats are grouped into three categories — `config`, `data`, `document`. The converter dispatches primarily by matching `source.category()` == `target.category()`. The only cross-category conversions are JSON/XML ↔ CSV (routed through the data module). All other cross-category conversions (e.g., JSON→PDF) are rejected with `UnsupportedConversion`. This is enforced in both Rust (`converter/mod.rs`) and the frontend (`VALID_CONVERSIONS`).

**PDF is output-only**: `Format::Pdf` returns an empty `valid_targets()` — there are no conversions from PDF to any other format. The frontend's `VALID_CONVERSIONS` mirrors this (`pdf: []`). The frontend source format picker excludes PDF explicitly.

**Overwrite safety**: The GUI `commands.rs` returns `OutputExists` error if the output file already exists (prevents silent data loss). The CLI does not check, following Unix conventions.

**i18n**: The frontend supports English (`en`) and Chinese (`zh`) via `src/i18n/`. The `useI18n()` composable returns `{ locale, t }` with `{param}` interpolation and localStorage persistence under key `vert-locale`. Fallback is `en`. All UI strings in components use `t('key')` — never hardcode user-visible text.

**Config conversion strategy**: All config formats (Properties/YAML/JSON/TOML/XML) convert via `serde_json::Value` as intermediate — O(n+m) instead of O(n×m) direct converters.

**Data conversion strategy**: CSV/JSON/XML table data converts via `Vec<HashMap<String,String>>` (rows of key-value pairs) as the intermediate representation.

**Document conversions**: Pure Rust throughout.
- **DOCX → PDF/HTML/MD**: `zip` unpacks the DOCX, `quick-xml` parses `word/document.xml` extracting text runs with formatting (bold/italic/headings). Renders via `genpdf` (PDF), inline HTML generation, or inline Markdown generation.
- **MD → HTML/PDF**: `pulldown-cmark` parses Markdown to events. HTML output via `push_html`. PDF output via `genpdf` with style tracking (bold/italic/headings).
- **HTML → PDF/MD**: `scraper` parses HTML with CSS selectors. PDF via `genpdf`, Markdown via direct text extraction with tag-to-Markdown mapping.
Text-focused; complex tables and images are simplified.

**Tauri plugins**: `tauri-plugin-opener` — opens output files in the OS default app (`openPath`). `tauri-plugin-dialog` — native file browse dialog for selecting input files.

**Feature flags**: The `gui` feature (on by default) gates Tauri dependencies. The CLI binary (`vert-cli`) builds with `--no-default-features` so it doesn't require GTK/WebKit system libraries. The library crate (`vert_lib`) uses `#[cfg(feature = "gui")]` to conditionally compile the Tauri app entry point and commands — the converter module is always available regardless.

## Important constraints

- **`src/utils/formats.ts` VALID_CONVERSIONS must stay in sync with Rust `Format::valid_targets()` in `src-tauri/src/converter/traits.rs`.** These are two independent sources of truth for which conversions are allowed. When adding a new conversion path, update both.

### Checklist for adding a new format

Adding a format touches **8 files** across both stacks:

| # | File | Change |
|---|---|---|
| 1 | `src/types/index.ts` | Add to `FileFormat` union |
| 2 | `src/utils/formats.ts` | Add entries in `FORMAT_EXTENSIONS`, `FORMAT_LABELS`, `FORMAT_CATEGORY`, `VALID_CONVERSIONS` |
| 3 | `src/components/FormatSelector.vue` | Add to `sourceFormats` array (omit if output-only like PDF) |
| 4 | `src/components/FileDropZone.vue` | Add extension to `browseFile()` filter |
| 5 | `src-tauri/src/converter/traits.rs` | Add variant to `Format` enum + update `from_extension`, `from_str`, `extension`, `valid_targets`, `category`, `Display` |
| 6 | `src-tauri/src/converter/mod.rs` | Add dispatch arm in `convert_file()` |
| 7 | Domain module (`config.rs`/`data.rs`/`document.rs`) | Add parse + serialize functions and dispatch arms |
| 8 | `src-tauri/src/cli.rs` `list()` | Add format to the category listing |

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
