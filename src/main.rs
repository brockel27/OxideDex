mod display;
mod dual_display;
mod format;
mod type_matchup;

use crate::display::display_pokemon_data;
use crate::dual_display::display_dual;

use colored::*;
use rand::Rng;
use rustemon::client::RustemonClient;
use std::env;

#[tokio::main]
async fn main() {
    clearscreen::clear().expect("Failed to clear screen");
    // Force ANSI color codes even when stdout isn't a TTY (e.g. piped output).
    control::set_override(true);

    let args: Vec<String> = env::args().collect();
    let client = RustemonClient::default();

    // Parse args: -s/-shiny/--shiny is positional and marks the preceding name as shiny.
    // If the flag comes before any name, it marks the next name instead.
    let resolve = |s: &str| -> String {
        if s == "random" { rand::thread_rng().gen_range(1..=1025).to_string() } else { s.to_string() }
    };
    let is_shiny_flag = |s: &str| matches!(s, "-s" | "-shiny" | "--shiny");

    let mut entries: Vec<(String, bool)> = Vec::new();
    let mut next_shiny = false;
    for arg in &args[1..] {
        if is_shiny_flag(arg.as_str()) {
            if let Some(last) = entries.last_mut() { last.1 = true; } else { next_shiny = true; }
        } else {
            entries.push((resolve(&arg.to_lowercase()), next_shiny));
            next_shiny = false;
        }
    }

    if entries.len() == 2 {
        let (shiny_a, shiny_b) = (entries[0].1, entries[1].1);
        if let Err(msg) = display_dual(&entries[0].0, &entries[1].0, &client, shiny_a, shiny_b).await {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
    } else if entries.len() == 1 {
        if let Err(msg) = display_pokemon_data(&entries[0].0, &client, entries[0].1).await {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
    } else {
        eprintln!("Usage: OxideDex <pokemon> [-s] [<pokemon2> [-s]]");
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
