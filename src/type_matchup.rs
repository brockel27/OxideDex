use std::collections::HashMap;
use crate::format::{colorize_type, visible_len};
use crate::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use colored::*;

const TYPE_NAMES: [&str; 18] = [
    "normal", "fighting", "flying", "poison", "ground",
    "rock", "bug", "ghost", "steel", "fire",
    "water", "grass", "electric", "psychic", "ice",
    "dragon", "dark", "fairy",
];

// Fetches damage relations for each of the Pokémon's types and returns a map of type → damage multiplier.
pub async fn type_hash(p: &Pokemon, client: &RustemonClient) -> HashMap<String, f32> {
    let mut p_types: HashMap<String, f32> = TYPE_NAMES.iter().map(|t| (t.to_string(), 1.0)).collect();

    for type_slot in &p.types {
        let type_data = rustemon::pokemon::type_::get_by_name(type_slot.type_.name.as_str(), client).await.unwrap();
        for entry in type_data.damage_relations.double_damage_from {
            if let Some(v) = p_types.get_mut(&entry.name) { *v *= 2.0; }
        }
        for entry in type_data.damage_relations.half_damage_from {
            if let Some(v) = p_types.get_mut(&entry.name) { *v *= 0.5; }
        }
        for entry in type_data.damage_relations.no_damage_from {
            if let Some(v) = p_types.get_mut(&entry.name) { *v *= 0.0; }
        }
    }

    p_types
}

// Converts a damage multiplier to its display string (e.g. 0.5 → "½×", 4.0 → "4×").
fn format_mult(mult: f32) -> String {
    if      mult == 0.0  { "0×".to_string() }
    else if mult == 0.25 { "¼×".to_string() }
    else if mult == 0.5  { "½×".to_string() }
    else if mult == 1.0  { "1×".to_string() }
    else if mult == 2.0  { "2×".to_string() }
    else if mult == 4.0  { "4×".to_string() }
    else                 { format!("{}×", mult) }
}

// Colors a multiplier string by effectiveness: white=immune, green=resists, grey=neutral, orange=weak, red=double weak.
fn colorize_multiplier(mult: f32) -> ColoredString {
    let s = format_mult(mult);
    if      mult == 0.0  { s.white().bold() }
    else if mult == 0.25 { s.truecolor(0, 200, 80).bold() }
    else if mult == 0.5  { s.green().bold() }
    else if mult == 1.0  { s.truecolor(150, 150, 150).normal() }
    else if mult == 2.0  { s.truecolor(255, 128, 0).bold() }
    else if mult == 4.0  { s.red().bold() }
    else                 { s.normal() }
}

// Prints a 3×6 bordered grid of all 18 types with color-coded damage multipliers.
pub fn print_type_matchup(matchup: &HashMap<String, f32>) {
    const COLS: usize = 3;
    const ROWS: usize = 6;
    // visible cell width: "| " (2) + name padded to 9 + " " (1) + mult (2) + " " (1) = 15
    const TOTAL_W: usize = COLS * 15 + 1;

    let eq   = "=".red().to_string();
    let pipe = "|".red().to_string();
    let sep  = eq.repeat(TOTAL_W);

    println!("{}", sep);
    println!("{} {:<w$} {}", pipe, "Type Effectiveness", pipe, w = TOTAL_W - 4);
    println!("{}", sep);

    for row in 0..ROWS {
        let mut line = String::new();
        for col in 0..COLS {
            let type_name = TYPE_NAMES[row * COLS + col];
            let mult = *matchup.get(type_name).unwrap_or(&1.0);
            let colored_name = colorize_type(type_name).to_string();
            let name_pad = " ".repeat(9_usize.saturating_sub(visible_len(&colored_name)));
            let mult_str = colorize_multiplier(mult).to_string();
            line.push_str(&pipe);
            line.push(' ');
            line.push_str(&colored_name);
            line.push_str(&name_pad);
            line.push(' ');
            line.push_str(&mult_str);
            line.push(' ');
        }
        line.push_str(&pipe);
        println!("{}", line);
    }

    println!("{}", sep);
}
