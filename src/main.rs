mod display;

use crate::display::*;
use colored::*;
use rustemon::client::RustemonClient;
use std::env;

#[tokio::main]
async fn main() {
    clearscreen::clear().expect("Failed to clear screen");
    // Ignore terminal settings for color output
    control::set_override(true);
    // std::env::args() doen't allow access to uninitialized memory
    let args: Vec<String> = env::args().collect();

    // Initialize the client
    let client = RustemonClient::default();

    if args.len() == 2 {
        let pokemon_name = args[1].to_lowercase();
        display_pokemon_data(&pokemon_name, &client).await;
    } else {
        eprintln!("Usage: rustdex <pokemon_name_or_id>");
    }
}

// use for testing: cargo test print_color_palette -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_color_palette() {
        let types = vec![
            "fire", "water", "grass", "electric", "ice", "fighting", "poison", "ground", "flying",
            "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
        ];

        println!("\n--- Pokedex Color Palette ---");
        for t in types {
            println!("{}: {}", t, colorize_type(t));
        }
    }
}
