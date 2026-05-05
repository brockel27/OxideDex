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

## Overview

**OxideDex** leverages the PokéAPI to provide real-time sprite display, base stats, type effectiveness, and abilities with a color-coded terminal interface. Look up a single Pokémon or compare two side by side.

---

## Features

- **Single display** — sprite, info box (name, dex number, height, weight, types, abilities, generation), stat bars with color-coded BST visualization, and a full 18-type effectiveness grid
- **Dual display** — two Pokémon side by side with their sprites composited into one image, individual info/stat columns, and per-Pokémon type effectiveness grids
- **Responsive layout** — dual display automatically scales column widths to fit your terminal, truncating long ability lists gracefully when needed
- **Type effectiveness** — multiplicative dual-type stacking (0×, ¼×, ½×, 1×, 2×, 4×) rendered in a 3×6 color-coded grid
- **Local API cache** — responses are cached via rustemon so repeat lookups are near-instant
- **Truecolor styling** — type names, stat bars, and effectiveness multipliers are all color-coded; decorative box-drawing borders

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

# Compare two Pokémon side by side
cargo run -- charizard blastoise
cargo run -- arcanine arcanine-hisui
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

<div align="center">
<sub>Built with ⚙️ and ❤️ as a Rust learning project</sub>
</div>
