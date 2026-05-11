<div align="center">

```
 ██████╗ ██╗  ██╗██╗██████╗ ███████╗██████╗ ███████╗██╗  ██╗
██╔═══██╗╚██╗██╔╝██║██╔══██╗██╔════╝██╔══██╗██╔════╝╚██╗██╔╝
██║   ██║ ╚███╔╝ ██║██║  ██║█████╗  ██║  ██║█████╗   ╚███╔╝ 
██║   ██║ ██╔██╗ ██║██║  ██║██╔══╝  ██║  ██║██╔══╝   ██╔██╗ 
╚██████╔╝██╔╝ ██╗██║██████╔╝███████╗██████╔╝███████╗██╔╝ ██╗
 ╚═════╝ ╚═╝  ╚═╝╚═╝╚═════╝ ╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═╝
```

<img src="OxideDex_logo.png" alt="OxideDex Logo" width="220"/>

*A high-performance, terminal-based Pokédex CLI built with Rust*

![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
![PokeAPI](https://img.shields.io/badge/data-PokéAPI-red?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-grey?style=flat-square)

</div>

---

> **experimental branch** — This branch is used for messing around with [Claude Code](https://claude.ai/code) and testing out new features. Things here may be unpolished or in flux.

---

## Overview

**OxideDex** leverages the PokéAPI to provide real-time sprite display, base stats, type effectiveness, and abilities with a color-coded terminal interface. Look up a single Pokémon, compare two side by side, or let the Dex surprise you with a random pick.

---

## Features

- **Single display** — sprite, info box (name, dex number, height, weight, types, abilities, generation), stat bars with color-coded BST visualization, and a full 18-type effectiveness grid
- **Dual display** — two Pokémon side by side with their sprites composited into one image, individual info/stat columns, and per-Pokémon type effectiveness grids
- **Pokédex flavor text** — a randomly selected game description shown at the bottom of every lookup (or side-by-side in dual mode)
- **Alternate Forms** — non-default varieties (Megas, Gigantamax, regional forms, cosplay caps) and appearance-only forms (Furfrou trims, Burmy cloaks, Unown letters) are listed together, truncated to 6 with `...` for Pokémon with many variants
- **Form-name lookup** — pass a specific form slug to display that variant with its own sprite (e.g. `furfrou-heart`, `burmy-sandy`, `charizard-mega-x`); appearance-only forms that have no separate Pokémon entry are resolved automatically via the form endpoint
- **Shiny sprites** — add `-s` after a Pokémon name to display its shiny sprite; in dual display each slot is independently controlled
- **Random mode** — use `random` as a name to pull a surprise Pokémon from the full Dex (#1–1025)
- **Responsive layout** — dual display automatically scales column widths to fit your terminal, truncating long ability lists gracefully when needed
- **Type effectiveness** — multiplicative dual-type stacking (0×, ¼×, ½×, 1×, 2×, 4×) rendered in a 3×6 color-coded grid
- **Local API cache** — responses are cached via rustemon so repeat lookups are near-instant
- **Truecolor styling** — type names, stat bars, and effectiveness multipliers are all color-coded; decorative box-drawing borders
- **Native-size sprites** — small sprites (older generations) render at their natural pixel size and are never upscaled

---

## Tech Stack

| Crate | Purpose |
|---|---|
| `rustemon` | PokéAPI client with local disk cache |
| `tokio` | Async runtime |
| `colored` | Truecolor ANSI styling |
| `reqwest` | HTTP sprite fetching |
| `image` | PNG decoding, cropping, compositing |
| `viuer` | Inline terminal image rendering |
| `crossterm` | Terminal size detection for responsive layout |
| `bytes` | Raw byte buffer for sprite downloads |
| `rand` | Random Pokémon selection and flavor text sampling |

---

## Installation

Requires the Rust toolchain ([install here](https://rustup.rs/)).

```bash
# Clone the repository
git clone https://github.com/brockel27/OxideDex.git
cd OxideDex

# Build the project
cargo build --release

# Run the program
./target/release/OxideDex pikachu
```

---

## Usage

```bash
# Single Pokémon — by name or National Dex ID
cargo run -- lucario
cargo run -- 448

# Shiny sprite
cargo run -- pikachu -s

# Random Pokémon
cargo run -- random

# Random shiny
cargo run -- random -s

# Compare two Pokémon side by side
cargo run -- charizard blastoise

# Dual display with shiny — -s is positional, applies to the preceding name
cargo run -- pikachu -s pikachu          # left shiny, right normal
cargo run -- pikachu pikachu -s          # left normal, right shiny
cargo run -- pikachu -s pikachu -s       # both shiny

# Random slots work in dual display too
cargo run -- random random -s

# Specific alternate form — appearance-only forms (Furfrou trims, Burmy cloaks, etc.)
cargo run -- furfrou-heart
cargo run -- burmy-sandy

# Specific alternate form — mechanically distinct varieties (Megas, regional forms, etc.)
cargo run -- charizard-mega-x
cargo run -- pikachu-original-cap
```

---

## Terminal Requirements

A truecolor terminal is recommended for the best experience. Dual display works best at 160+ columns; the layout adapts automatically but narrower terminals will truncate some fields.

| Platform | Recommended |
|---|---|
| Windows | Windows Terminal (not cmd.exe or legacy PowerShell) |
| macOS | iTerm2 or Terminal.app (macOS 10.15+) |
| Linux/WSL | Any modern emulator with `COLORTERM=truecolor` |

Verify truecolor support:

```bash
echo $COLORTERM  # should print "truecolor"

# Verify color output
cargo test print_color_palette -- --nocapture
```

---

## Sources

- [Rustemon docs](https://docs.rs/rustemon/latest/rustemon/)
- [Cargo Cheatsheet](https://github.com/johnnysecond/rust-cargo-cheatsheet)

---

## AI Assistance

This project was developed with the assistance of [Claude Code](https://claude.ai/code) (Anthropic). Claude was used as a coding assistant throughout development — helping with implementation, debugging, and design decisions. All code was reviewed and directed by the project author.

---

<div align="center">
<sub>Built with ⚙️ and ❤️ as a Rust learning project</sub>
</div>
