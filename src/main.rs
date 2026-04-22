mod display;
mod format;

use crate::display::*;
use colored::*;
use rustemon::client::RustemonClient;
use std::env;

#[tokio::main]
async fn main() {
    control::set_override(true);
    let args: Vec<String> = env::args().collect();
    let client = RustemonClient::default();

    if args.len() == 2 {
        let pokemon_name = args[1].to_lowercase();
        display_pokemon_data(&pokemon_name, &client).await;
    } else {
        eprintln!("Usage: OxideDex <pokemon name or id>");
    }
}

#[cfg(test)]
mod tests {
    use crate::format::colorize_type;

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
