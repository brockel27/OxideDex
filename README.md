# OxideDex 🦀

A high-performance, terminal-based Pokedex CLI built with Rust.

OxideDex provides a sleek, color-coded interface for exploring Pokémon data directly from your terminal. It leverages the PokeAPI to provide real-time stats, types, and abilities with zero local database overhead.

## Features

    Lookup: Search by Pokémon name or National ID.

    Asynchronous Architecture: Powered by tokio and rustemon for non-blocking API requests.

    Formatted UI: Automatic title-casing and "kebab-case" cleaning for a polished look.

    Dynamic Color Palette: Historically accurate type colors optimized for modern terminal emulators.

    Unit Conversion: Automatic conversion of internal API units to standard Metric (Meters/Kilograms).

## Tech Stack & Environment

    Language: Rust

    Runtime: Tokio (Asynchronous I/O)

    API Wrapper: Rustemon

    UI/Styling: Colored

    Development Environment: Developed on a Surface Pro 7 (Windows 11) using WSL2 (Windows Subsystem for Linux) with a minimal Kali Linux distribution.

## Installation

To build OxideDex from source, you will need the Rust toolchain (Cargo) installed.

    Clone the repository:
    Bash

    git clone https://github.com/brockel27/OxideDex.git
    cd rustdex

    Build the project:
    cargo build --release

    Run the program:
    ./target/release/OxideDex pikachu

## Terminal Requirements
For the best experience, a truecolor terminal is recommended.
- **Windows:** Windows Terminal (recommended), not cmd.exe or old PowerShell
- **macOS:** iTerm2 or the default Terminal.app on macOS 10.15+
- **Linux/WSL:** Any modern terminal emulator with COLORTERM=truecolor set

To verify your terminal supports truecolor:
echo $COLORTERM  # should print "truecolor"

Verify color output by running:
    '''cargo test print_color_palette -- --nocapture'''
<img width="696" height="655" alt="image" src="https://github.com/user-attachments/assets/bf4420a7-ec52-40b9-a913-00cd5f3fece6" />


## Usage

You can run the program using cargo run followed by the Pokémon name or ID:

### Search by name
cargo run -- lucario

### Search by ID
cargo run -- 448

## Academic Context

This project was developed for an undergraduate level Programming Languages course to demonstrate proficiency in:

    Ownership & Borrowing: Managing memory safely in a systems-level language.

    Asynchronous I/O: Handling network requests without blocking the main execution thread.

    Data Pipelines: Transforming raw JSON data into user-friendly, formatted terminal output.


## Sources:
Rustemon: https://docs.rs/rustemon/latest/rustemon/
Cargo CheatSheet: https://github.com/johnnysecond/rust-cargo-cheatsheet
