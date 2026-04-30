use rustemon::model::pokemon::Pokemon;
use colored::*;
use image::{DynamicImage, GenericImageView};

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

pub fn types_to_string(p: &Pokemon) -> String {
    p.types
        .iter()
        .map(|ptype| colorize_type(&ptype.type_.name).to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_name(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn format_stat_name(name: &str) -> String {
    match name {
        "hp"               => name.to_uppercase(),
        "attack" |
        "defense" |
        "speed"            => format_name(name),
        "special-attack"   => "Sp. Atk".to_string(),
        "special-defense"  => "Sp. Def".to_string(),
        _                  => name.to_string(),
    }
}

pub fn format_generation(gen_name: &str) -> String {
    gen_name
        .strip_prefix("generation-")
        .map(|roman| format!("{}", roman.to_uppercase()))
        .unwrap_or_else(|| format_name(gen_name))
}

pub fn is_transparent(img: &DynamicImage, start: u32, is_row: bool) -> bool {
    if is_row {
        (0..img.width()).all(|x| img.get_pixel(x, start).0[3] == 0)
    } else {
        (0..img.height()).all(|y| img.get_pixel(start, y).0[3] == 0)
    }
}
