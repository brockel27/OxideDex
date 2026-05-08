mod display;
mod dual_display;
mod format;
mod type_matchup;

use crate::display::display_pokemon_data;
use crate::dual_display::display_dual;

use colored::*;
use rustemon::client::RustemonClient;
use std::env;

#[tokio::main]
async fn main() {
    clearscreen::clear().expect("Failed to clear screen");
    // Force ANSI color codes even when stdout isn't a TTY (e.g. piped output).
    control::set_override(true);

    let args: Vec<String> = env::args().collect();
    let client = RustemonClient::default();

    if args.len() == 3 {
        let pokemon_a = args[1].to_lowercase();
        let pokemon_b = args[2].to_lowercase();
        if let Err(msg) = display_dual(&pokemon_a, &pokemon_b, &client).await {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
    } else if args.len() == 2 {
        let pokemon_name = args[1].to_lowercase();
        if let Err(msg) = display_pokemon_data(&pokemon_name, &client).await {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
    } else {
        eprintln!("Usage: OxideDex <pokemon_name_or_id> [<pokemon2_name_or_id>]");
    }

}

#[cfg(test)]
mod tests {
    use crate::format::colorize_type;

    // Prints every type color to stdout so the palette can be visually verified.
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
