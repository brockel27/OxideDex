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

**OxideDex** leverages the PokéAPI to provide real-time sprite display, stats, types, and abilities with a color-coded terminal interface.

---

## Tech Stack

| Crate | Purpose |
|---|---|
| `rustemon` | PokéAPI client |
| `tokio` | Async runtime |
| `colored` | Terminal type color styling |
| `reqwest` | HTTP sprite fetching |
| `image` | Image decoding |
| `viuer` | Terminal sprite rendering |

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
# Search by name
cargo run -- lucario

# Search by ID
cargo run -- 448
```

---

## Terminal Requirements

A truecolor terminal is recommended for the best experience.

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
