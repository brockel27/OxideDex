use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::pokemon::pokemon;
use std::env;
use colored::*;

async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient) {
    // Rustemon's get_by_name handles the API request and JSON parsing
    match pokemon::get_by_name(pokemon_name, client).await {
        Ok(p) => {
            // Convert the integers from the API to floats
            // Height orignally in decimeters (dm), Weight in hectograms (hg)
            let height_in_meters = p.height as f32 / 10.0;
            let weight_in_kg = p.weight as f32 / 10.0;

            // Map the abilities into a readable String, originally a Vec
            let abilities_list: String = p.abilities
               .iter()
               .filter_map(|a| a.ability.as_ref())
               .map(|ability| format_name(&ability.name))
               .collect::<Vec<_>>()
               .join(", ");                

            let formatted_name = format_name(&p.name);

             println!("Name: {}", formatted_name);
             println!("Height: {} m", height_in_meters);
             println!("Weight: {} kg", weight_in_kg);
             println!("Types: {}", types_to_string(&p));
             println!("Abilities: {}", abilities_list); 
         }
         Err(error) => {
             eprintln!("Error: Could not find '{}'. ({})", pokemon_name, error);
        }
    }
}

fn types_to_string(p: &Pokemon) -> String {
    p.types
        .iter()
        .map(|ptype| {
            let name = &ptype.type_.name;
            // Format the name first (e.g., "fire"), then colorize it
            colorize_type(name).to_string() 
         })
        .collect::<Vec<_>>()
        .join(", ")
}


fn format_name(name: &str) -> String {
    name.split('-') // Split at hyphens
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                // Capitalize first letter, keep the rest as is
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ") // Join back with spaces
}

// Helper to apply colors based on the type name
fn colorize_type(type_name: &str) -> ColoredString {
    let formatted = format_name(type_name);
    match type_name.to_lowercase().as_str() {
        "fire" => formatted.red().bold(),
        "water" => formatted.blue().bold(),
        "grass" => formatted.green().bold(),
        "electric" => formatted.yellow().bold(),
        "ice" => formatted.cyan().bold(),
        "poison" => formatted.magenta().bold(),
        "fighting" => formatted.truecolor(255, 128, 0).bold(),
        "ground" => formatted.truecolor(226, 191, 101).bold(),
        "flying" => formatted.truecolor(184, 143, 243).bold(),
        "psychic" => formatted.truecolor(249, 85, 135).bold(),
        "bug" => formatted.truecolor(166, 185, 26).bold(),
        "rock" => formatted.truecolor(182, 161, 54).bold(),
        "ghost" => formatted.truecolor(115, 87, 151).bold(),
        "dragon" => formatted.truecolor(111, 53, 252).bold(),
        "dark" => formatted.truecolor(112, 87, 70).bold(),
        "steel" => formatted.truecolor(183, 183, 206).bold(),
        "fairy" => formatted.truecolor(214, 133, 173).bold(),
        _ => formatted.normal(), 
    }
}

#[tokio::main]
async fn main() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_color_palette() {
        let types = vec![
            "fire", "water", "grass", "electric", "ice", "fighting", 
            "poison", "ground", "flying", "psychic", "bug", "rock", 
            "ghost", "dragon", "dark", "steel", "fairy"
        ];

        println!("\n--- Pokedex Color Palette ---");
        for t in types {
            println!("{}: {}", t, colorize_type(t));
        }
    }
}
