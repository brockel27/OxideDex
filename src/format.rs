use rustemon::model::pokemon::Pokemon;
use colored::*;

// Helper to apply colors based on the type name
pub fn colorize_type(type_name: &str) -> ColoredString {
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

pub fn format_name(name: &str) -> String {
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

pub fn types_to_string(p: &Pokemon) -> String {
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
