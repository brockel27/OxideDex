use crate::format::*;
use crate::type_matchup::{type_hash, build_type_matchup_lines};

use rand::seq::SliceRandom;
use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::pokemon::{pokemon, pokemon_species};
use std::io::Cursor;
use colored::Colorize;

const DEFAULT_COL_W: usize = 58;
const BAR_WIDTH:     usize = 24;

// Downloads raw PNG sprite bytes from a URL.
pub(crate) async fn fetch_sprite(url: &str) -> Option<bytes::Bytes> {
    match reqwest::get(url).await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => Some(bytes),
            Err(e) => { eprintln!("Could not download sprite: {}", e); None }
        },
        Err(e) => { eprintln!("Could not fetch sprite: {}", e); None }
    }
}

// Trims transparent borders, re-pads, and renders a sprite inline in the terminal.
fn display_sprite(bytes: bytes::Bytes, text_width: usize) {
    const RENDER_WIDTH: u32 = 128;
    match image::load(Cursor::new(bytes), image::ImageFormat::Png) {
        Ok(img) => {
            let (mut top, mut bottom) = (0, img.height() - 1);
            let (mut left, mut right) = (0, img.width() - 1);
            while top < bottom && is_row_transparent(&img, top) { top += 1; }
            while bottom > top && is_row_transparent(&img, bottom) { bottom -= 1; }
            while left < right && is_col_transparent(&img, left) { left += 1; }
            while right > left && is_col_transparent(&img, right) { right -= 1; }
            let trimmed = img.crop_imm(left, top, right - left + 1, bottom - top + 1);

            let pad = 4u32;
            let trimmed_rgba = trimmed.to_rgba8();
            let mut padded = image::RgbaImage::new(
                trimmed_rgba.width() + pad * 2,
                trimmed_rgba.height() + pad * 2,
            );
            image::imageops::overlay(&mut padded, &trimmed_rgba, pad as i64, pad as i64);

            let x_offset = (text_width as u32).saturating_sub(RENDER_WIDTH) / 2;
            let config = viuer::Config {
                transparent: true,
                absolute_offset: false,
                x: x_offset as u16,
                width: Some(RENDER_WIDTH),
                use_kitty: false,
                use_iterm: false,
                ..Default::default()
            };
            let _ = viuer::print(&image::DynamicImage::from(padded), &config);
        }
        Err(e) => eprintln!("Could not decode image: {}", e),
    }
}

// Fetches a random English Pokédex flavor text entry for a species.
pub async fn get_flavor_text(species_name: &str, client: &RustemonClient) -> Option<String> {
    let species = pokemon_species::get_by_name(species_name, client).await.ok()?;
    let english: Vec<_> = species
        .flavor_text_entries
        .iter()
        .filter(|e| e.language.name == "en")
        .collect();
    let entry = english.choose(&mut rand::thread_rng())?;
    let cleaned = entry
        .flavor_text
        .replace(['\n', '\r', '\x0c'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    Some(cleaned)
}

// Wraps flavor text into lines of at most max_w visible chars.
pub fn wrap_text(text: &str, max_w: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_w {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }
    if !current.is_empty() { lines.push(current); }
    lines
}

// Right-aligns `right` within `col_w`, with `left` at the start.
fn right_align(left: &str, right: &str, col_w: usize) -> String {
    let left_vis  = visible_len(left);
    let right_vis = visible_len(right);
    let gap = col_w.saturating_sub(left_vis + right_vis).max(1);
    format!("{}{:>gap$}{}", left, "", right, gap = gap)
}

// Builds a single stat bar line.
fn stat_line(stat_name: &str, value: i64, col_w: usize) -> String {
    let bar_w   = BAR_WIDTH.min(col_w.saturating_sub(17));
    let bar_len = ((value as f32 / 180.0 * bar_w as f32) as usize).min(bar_w);
    let filled  = colorize_line(&"█".repeat(bar_len), &value).to_string();
    let empty   = "░".repeat(bar_w - bar_len)
        .truecolor(EMPTY_BAR_CLR.0, EMPTY_BAR_CLR.1, EMPTY_BAR_CLR.2)
        .to_string();
    let label = format!("{:<8}", stat_name)
        .truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2)
        .to_string();
    format!("  {}  {:>3}   {}{}", label, value, filled, empty)
}

// Builds the stacked display lines for a Pokémon (identity header + physical info + stats).
// col_w is the exact inner width of the enclosing border column.
pub async fn pokemon_display_lines(p: &Pokemon, client: &RustemonClient, col_w: usize) -> Vec<String> {
    let generation_str = match pokemon_species::get_by_name(&p.species.name, client).await {
        Ok(species) => format_generation(&species.generation.name),
        Err(_) => String::from("?"),
    };

    let height_m  = p.height as f32 / 10.0;
    let weight_kg = p.weight as f32 / 10.0;

    let name      = format_name(&p.name).bold().to_string();
    let dex_id    = format!("#{}", p.id)
        .truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2).to_string();
    let gen_str   = format!("Gen {}", generation_str)
        .truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2).to_string();
    let types_str = types_to_string(p);

    let abilities_str = p.abilities.iter()
        .filter_map(|a| a.ability.as_ref())
        .map(|a| format_name(&a.name))
        .collect::<Vec<_>>()
        .join("  ·  ");

    let h_label = "Height".truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2).to_string();
    let w_label = "Weight".truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2).to_string();
    let a_label = "Abilities".truecolor(LABEL_CLR.0, LABEL_CLR.1, LABEL_CLR.2).to_string();
    let dot     = "   ·   ".truecolor(RULE_CLR.0, RULE_CLR.1, RULE_CLR.2).to_string();

    let physical_line = format!("{}  {:.1} m{}{}  {:.1} kg",
        h_label, height_m, dot, w_label, weight_kg);
    let ability_line  = format!("{}   {}", a_label, abilities_str);

    let base_stat_total: i64 = p.stats.iter().map(|s| s.base_stat).sum();
    let bst_right = format!("BST  {}", base_stat_total).bold().to_string();
    let bst_line  = right_align("", &bst_right, col_w);

    let mut lines: Vec<String> = Vec::new();

    // Identity header
    lines.push(right_align(&name, &dex_id, col_w));
    lines.push(right_align(&types_str, &gen_str, col_w));
    lines.push(plain_rule(col_w));

    // Physical info
    lines.push(physical_line);
    lines.push(ability_line);
    lines.push(String::new());

    // Base stats
    lines.push(section_rule("Base Stats", col_w));
    for s in &p.stats {
        lines.push(stat_line(&format_stat_name(&s.stat.name), s.base_stat, col_w));
    }
    lines.push(bst_line);
    lines.push(String::new());

    lines
}

// Fetches a Pokémon by name, renders its sprite, and prints its info.
pub async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient, shiny: bool) -> Result<(), String> {
    let p = pokemon::get_by_name(pokemon_name, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_name, e))?;

    let display_lines = pokemon_display_lines(&p, client, DEFAULT_COL_W).await;
    let matchup       = type_hash(&p, client).await;
    let matchup_lines = build_type_matchup_lines(&matchup, DEFAULT_COL_W);
    let flavor_text   = get_flavor_text(&p.species.name, client).await;

    println!("{}", border_top(DEFAULT_COL_W));
    let sprite_url = if shiny { p.sprites.front_shiny.as_deref() } else { p.sprites.front_default.as_deref() };
    if let Some(url) = sprite_url {
        if let Some(bytes) = fetch_sprite(url).await {
            display_sprite(bytes, DEFAULT_COL_W + 4);
        }
    }
    println!("{}", border_row("", DEFAULT_COL_W));

    for line in &display_lines {
        println!("{}", border_row(line, DEFAULT_COL_W));
    }
    for line in &matchup_lines {
        println!("{}", border_row(line, DEFAULT_COL_W));
    }

    if let Some(text) = flavor_text {
        println!("{}", border_row(&section_rule("Pokédex", DEFAULT_COL_W), DEFAULT_COL_W));
        for line in wrap_text(&text, DEFAULT_COL_W - 2) {
            println!("{}", border_row(&line, DEFAULT_COL_W));
        }
        println!("{}", border_row("", DEFAULT_COL_W));
    }

    println!("{}", border_bottom(DEFAULT_COL_W));
    Ok(())
}
