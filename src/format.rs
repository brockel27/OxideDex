use rustemon::model::pokemon::Pokemon;
use colored::*;
use image::{DynamicImage, GenericImageView};

// Colors a stat bar string based on the stat's value.
pub fn colorize_line(stat_line: &str, stat_value: &i64) -> ColoredString {
    match stat_value {
        30..60   => stat_line.truecolor(255, 135, 0).bold(),
        60..80   => stat_line.yellow().bold(),
        80..100  => stat_line.truecolor(166, 195, 25).bold(),
        100..120 => stat_line.green().bold(),
        120..=255 => stat_line.cyan().bold(),
        _ => stat_line.normal(),
    }
}

// Returns the type name as a bold colored string matching its official type color.
pub fn colorize_type(type_name: &str) -> ColoredString {
    let formatted = format_name(type_name);
    match type_name.to_lowercase().as_str() {
        "fire"     => formatted.red().bold(),
        "water"    => formatted.blue().bold(),
        "grass"    => formatted.green().bold(),
        "electric" => formatted.yellow().bold(),
        "ice"      => formatted.cyan().bold(),
        "poison"   => formatted.magenta().bold(),
        "fighting" => formatted.truecolor(255, 128, 0).bold(),
        "ground"   => formatted.truecolor(226, 191, 101).bold(),
        "flying"   => formatted.truecolor(184, 143, 243).bold(),
        "psychic"  => formatted.truecolor(249, 85, 135).bold(),
        "bug"      => formatted.truecolor(166, 185, 26).bold(),
        "rock"     => formatted.truecolor(182, 161, 54).bold(),
        "ghost"    => formatted.truecolor(115, 87, 151).bold(),
        "dragon"   => formatted.truecolor(111, 53, 252).bold(),
        "dark"     => formatted.truecolor(112, 87, 70).bold(),
        "steel"    => formatted.truecolor(183, 183, 206).bold(),
        "fairy"    => formatted.truecolor(214, 133, 173).bold(),
        _          => formatted.normal(),
    }
}

// Returns the printable character count of a string, ignoring ANSI escape codes.
pub fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' { in_escape = true; }
        else if in_escape { if c == 'm' { in_escape = false; } }
        else { len += 1; }
    }
    len
}

// Formats a Pokémon's types as a comma-separated colored string.
pub fn types_to_string(p: &Pokemon) -> String {
    p.types
        .iter()
        .map(|ptype| colorize_type(&ptype.type_.name).to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

// Converts a hyphen-separated PokeAPI name to Title Case (e.g. "mr-mime" → "Mr Mime").
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

// Maps PokeAPI stat names to display abbreviations (e.g. "special-attack" → "Sp. Atk").
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

// Converts a PokeAPI generation name to its Roman numeral form (e.g. "generation-iv" → "IV").
pub fn format_generation(gen_name: &str) -> String {
    gen_name
        .strip_prefix("generation-")
        .map(|roman| roman.to_uppercase())
        .unwrap_or_else(|| format_name(gen_name))
}

// Truncates a string to max_len visible chars, appending "..." if it was cut.
pub fn truncate_display(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len.saturating_sub(3)).collect::<String>() + "..."
    }
}

const BCLR: (u8, u8, u8) = (165, 155, 140);

// Returns the top border line of the outer display frame.
pub fn border_top(inner_w: usize) -> String {
    format!("{}{}{}",
        "╔".truecolor(BCLR.0, BCLR.1, BCLR.2),
        "═".repeat(inner_w + 2).truecolor(BCLR.0, BCLR.1, BCLR.2),
        "╗".truecolor(BCLR.0, BCLR.1, BCLR.2))
}

// Returns the bottom border line of the outer display frame.
pub fn border_bottom(inner_w: usize) -> String {
    format!("{}{}{}",
        "╚".truecolor(BCLR.0, BCLR.1, BCLR.2),
        "═".repeat(inner_w + 2).truecolor(BCLR.0, BCLR.1, BCLR.2),
        "╝".truecolor(BCLR.0, BCLR.1, BCLR.2))
}

// Wraps a content string with side borders, padding to inner_w visible columns.
pub fn border_row(content: &str, inner_w: usize) -> String {
    let pad = " ".repeat(inner_w.saturating_sub(visible_len(content)));
    format!("{} {}{} {}",
        "║".truecolor(BCLR.0, BCLR.1, BCLR.2),
        content, pad,
        "║".truecolor(BCLR.0, BCLR.1, BCLR.2))
}

// Returns true if every pixel in row y of the image is fully transparent.
pub fn is_row_transparent(img: &DynamicImage, y: u32) -> bool {
    (0..img.width()).all(|x| img.get_pixel(x, y).0[3] == 0)
}

// Returns true if every pixel in column x of the image is fully transparent.
pub fn is_col_transparent(img: &DynamicImage, x: u32) -> bool {
    (0..img.height()).all(|y| img.get_pixel(x, y).0[3] == 0)
}
