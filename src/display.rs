use crate::format::*;
use crate::type_matchup::{type_hash, build_type_matchup_lines};

use rand::seq::SliceRandom;
use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::pokemon::{pokemon, pokemon_species};
use std::io::Cursor;
use colored::Colorize;

const SPRITE_COLS: usize = 96;
const SIDE_GAP:    usize = 2;
const INFO_COL_W:  usize = 44;
const RIGHT_PAD:   usize = 4;
const FULL_COL_W:  usize = SPRITE_COLS + SIDE_GAP + INFO_COL_W + RIGHT_PAD;
const BAR_WIDTH:   usize = 26;

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

// Converts sprite PNG bytes into half-block ANSI strings (one string per two pixel rows).
fn sprite_to_lines(bytes: bytes::Bytes, target_cols: usize) -> Option<Vec<String>> {
    let img = image::load(Cursor::new(bytes), image::ImageFormat::Png).ok()?;

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

    // Only scale down — never upscale a sprite smaller than target_cols.
    let (resized, right_pad) = if padded.width() as usize > target_cols {
        let scale = target_cols as f32 / padded.width() as f32;
        let new_h = ((padded.height() as f32 * scale) as u32).max(1);
        let r = image::imageops::resize(&padded, target_cols as u32, new_h, image::imageops::FilterType::Lanczos3);
        (r, 0usize)
    } else {
        let pad = target_cols.saturating_sub(padded.width() as usize);
        (padded, pad)
    };

    let h = resized.height();
    let w = resized.width();
    let mut lines = Vec::new();
    let mut y = 0u32;
    while y < h {
        let mut row = String::new();
        for x in 0..w {
            let tp = resized.get_pixel(x, y);
            if y + 1 < h {
                let bp = resized.get_pixel(x, y + 1);
                let s = match (tp[3] >= 128, bp[3] >= 128) {
                    (false, false) => " ".to_string(),
                    (true,  false) => "▀".truecolor(tp[0], tp[1], tp[2]).to_string(),
                    (false, true)  => "▄".truecolor(bp[0], bp[1], bp[2]).to_string(),
                    (true,  true)  => "▄".on_truecolor(tp[0], tp[1], tp[2])
                                        .truecolor(bp[0], bp[1], bp[2]).to_string(),
                };
                row.push_str(&s);
            } else {
                let s = if tp[3] >= 128 {
                    "▀".truecolor(tp[0], tp[1], tp[2]).to_string()
                } else {
                    " ".to_string()
                };
                row.push_str(&s);
            }
        }
        row.push_str("\x1b[0m");
        if right_pad > 0 { row.push_str(&" ".repeat(right_pad)); }
        lines.push(row);
        y += 2;
    }
    Some(lines)
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
    let bar_w   = BAR_WIDTH.min(col_w.saturating_sub(18));
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

    lines.push(right_align(&name, &dex_id, col_w));
    lines.push(right_align(&types_str, &gen_str, col_w));
    lines.push(plain_rule(col_w));

    lines.push(physical_line);
    lines.push(ability_line);
    lines.push(String::new());
    lines.push(String::new());

    lines.push(section_rule("Base Stats", col_w));
    for s in &p.stats {
        lines.push(stat_line(&format_stat_name(&s.stat.name), s.base_stat, col_w));
    }
    lines.push(bst_line);
    lines.push(String::new());

    lines
}

// Fetches a Pokémon by name, renders its sprite side-by-side with info, and prints its data.
pub async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient, shiny: bool) -> Result<(), String> {
    let p = pokemon::get_by_name(pokemon_name, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_name, e))?;

    let sprite_url = if shiny { p.sprites.front_shiny.as_deref() } else { p.sprites.front_default.as_deref() };
    let sprite_lines: Vec<String> = match sprite_url {
        Some(url) => match fetch_sprite(url).await {
            Some(bytes) => sprite_to_lines(bytes, SPRITE_COLS).unwrap_or_default(),
            None => Vec::new(),
        },
        None => Vec::new(),
    };

    let info_lines    = pokemon_display_lines(&p, client, INFO_COL_W).await;
    let matchup       = type_hash(&p, client).await;
    let matchup_lines = build_type_matchup_lines(&matchup, INFO_COL_W);
    let flavor_text   = get_flavor_text(&p.species.name, client).await;

    // Assemble the full right column: identity/stats → type matchup → pokédex
    let mut right_col: Vec<String> = Vec::new();
    right_col.extend(info_lines);
    right_col.extend(matchup_lines);
    if let Some(text) = flavor_text {
        right_col.push(String::new());
        right_col.push(section_rule("Pokédex", INFO_COL_W));
        for line in wrap_text(&text, INFO_COL_W - 2) { right_col.push(line); }
        right_col.push(String::new());
    }

    let blank_sprite = " ".repeat(SPRITE_COLS);
    let max_lines = sprite_lines.len().max(right_col.len());

    println!("{}", border_top(FULL_COL_W));
    println!("{}", border_row("", FULL_COL_W));

    for i in 0..max_lines {
        let sprite_col = sprite_lines.get(i).map(|s| s.as_str()).unwrap_or(&blank_sprite);
        let info_col   = right_col.get(i).map(|s| s.as_str()).unwrap_or("");
        let combined   = format!("{}{:gap$}{}", sprite_col, "", info_col, gap = SIDE_GAP);
        println!("{}", border_row(&combined, FULL_COL_W));
    }

    println!("{}", border_bottom(FULL_COL_W));
    Ok(())
}
