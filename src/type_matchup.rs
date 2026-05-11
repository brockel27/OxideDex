use std::collections::HashMap;
use crate::format::{colorize_type, section_rule, visible_len, LABEL_CLR, RULE_CLR};
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

// Builds one or more lines for a type effectiveness category.
// Label is padded to LABEL_COL chars; types are joined with colored dots.
// Wraps to a new line (blank label column) if types overflow col_w.
fn build_category_row(label: &str, types: &[&str], col_w: usize) -> Vec<String> {
    const LABEL_COL: usize = 13;
    let dot = "  ·  ".truecolor(RULE_CLR.0, RULE_CLR.1, RULE_CLR.2).to_string();
    const DOT_VIS: usize = 5;

    let label_str = format!("{:<width$}", label, width = LABEL_COL)
        .truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2)
        .bold()
        .to_string();
    let indent = " ".repeat(LABEL_COL);

    if types.is_empty() {
        let dash = "—".truecolor(100, 100, 110).to_string();
        return vec![format!("{}{}", label_str, dash)];
    }

    let avail = col_w.saturating_sub(LABEL_COL + 2);
    let mut lines: Vec<String> = Vec::new();
    let mut current = label_str;
    let mut current_vis = LABEL_COL;
    let mut first_on_line = true;

    for type_name in types {
        let colored = colorize_type(type_name).to_string();
        let token_vis = visible_len(&colored);
        let needed = if first_on_line { token_vis } else { DOT_VIS + token_vis };

        if !first_on_line && current_vis + needed > LABEL_COL + avail {
            lines.push(current);
            current = indent.clone();
            current_vis = LABEL_COL;
            first_on_line = true;
        }

        if !first_on_line {
            current.push_str(&dot);
            current_vis += DOT_VIS;
        }
        current.push_str(&colored);
        current_vis += token_vis;
        first_on_line = false;
    }
    lines.push(current);
    lines
}

// Builds a grouped type effectiveness section: Immune / Resists / Weak / Double Weak.
// Neutral types are omitted. Each category always appears (showing — if empty).
pub fn build_type_matchup_lines(matchup: &HashMap<String, f32>, col_w: usize) -> Vec<String> {
    let mut immune:      Vec<&str> = Vec::new();
    let mut resists:     Vec<&str> = Vec::new();
    let mut weak:        Vec<&str> = Vec::new();
    let mut double_weak: Vec<&str> = Vec::new();

    for type_name in &TYPE_NAMES {
        let mult = *matchup.get(*type_name).unwrap_or(&1.0);
        if      mult == 0.0 { immune.push(type_name); }
        else if mult < 1.0  { resists.push(type_name); }
        else if mult == 2.0 { weak.push(type_name); }
        else if mult > 2.0  { double_weak.push(type_name); }
    }

    let mut lines = vec![section_rule("Type Effectiveness", col_w)];
    for row in build_category_row("Immune", &immune, col_w)      { lines.push(row); }
    for row in build_category_row("Resists", &resists, col_w)    { lines.push(row); }
    for row in build_category_row("Weak", &weak, col_w)          { lines.push(row); }
    for row in build_category_row("Double Weak", &double_weak, col_w) { lines.push(row); }
    lines.push(String::new());
    lines
}
