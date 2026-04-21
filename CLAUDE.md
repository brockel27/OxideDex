# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Run with a Pokémon name or National Dex ID
cargo run -- pikachu
cargo run -- 448

# Build release binary
cargo build --release
./target/release/OxideDex pikachu

# Run tests (use --nocapture to see terminal output)
cargo test
cargo test print_color_palette -- --nocapture
```

## Architecture

OxideDex is a single-binary async CLI. Entry point is `src/main.rs`, which initializes a `RustemonClient`, parses the CLI argument, and calls into `display.rs`.

**Module layout:**

- `src/main.rs` — arg parsing, tokio runtime entry, test module for the color palette
- `src/display.rs` — async functions: fetches the sprite from GitHub's PokeAPI sprite mirror, crops transparent borders with `viuer`, then fetches Pokémon data via `rustemon` and prints formatted output
- `src/format.rs` — pure formatting helpers: `colorize_type` (truecolor ANSI via `colored`), `format_name` (kebab-case → Title Case), `format_stat_name` (canonical stat abbreviations), `is_transparent` (pixel alpha check used for sprite trimming)
- `src/api_test.rs` — scratch/exploratory API test file

**Key data flow:**

1. `rustemon::pokemon::pokemon::get_by_name` → `rustemon` model struct
2. Sprite fetched separately via raw `reqwest` from GitHub sprites repo (not through `rustemon`)
3. All display formatting goes through `format.rs` helpers before printing

**Dependencies of note:**
- `rustemon` wraps PokeAPI; the client uses a local cache (`.rustemon-cache/` directory)
- `viuer` renders images inline in the terminal; requires a truecolor terminal for best results
- `colored` with `control::set_override(true)` forces ANSI color codes regardless of terminal detection
- Height/weight from PokeAPI are in decimeters and hectograms respectively; `display.rs` converts to meters/kg
