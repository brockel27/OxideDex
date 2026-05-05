use crate::format::*;
use crate::type_matchup::{type_hash, print_type_matchup};

use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::pokemon::{pokemon, pokemon_species};
use std::io::Cursor;
use colored::Colorize;

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
fn display_sprite(bytes: bytes::Bytes) {
    match image::load(Cursor::new(bytes), image::ImageFormat::Png) {
        Ok(img) => {
            let (mut top, mut bottom) = (0, img.height() - 1);
            let (mut left, mut right) = (0, img.width() - 1);
            while top < bottom && is_row_transparent(&img, top) { top += 1; }
            while bottom > top && is_row_transparent(&img, bottom) { bottom -= 1; }
            while left < right && is_col_transparent(&img, left) { left += 1; }
            while right > left && is_col_transparent(&img, right) { right -= 1; }
            let trimmed = img.crop_imm(left, top, right - left + 1, bottom - top + 1);

            // Transparent border prevents bilinear resize from blending edge pixels, causing fringe.
            let pad = 4u32;
            let trimmed_rgba = trimmed.to_rgba8();
            let mut padded = image::RgbaImage::new(
                trimmed_rgba.width() + pad * 2,
                trimmed_rgba.height() + pad * 2,
            );
            image::imageops::overlay(&mut padded, &trimmed_rgba, pad as i64, pad as i64);

            let config = viuer::Config {
                transparent: true,
                absolute_offset: false,
                width: Some(128),
                use_kitty: false,
                use_iterm: false,
                ..Default::default()
            };
            let _ = viuer::print(&image::DynamicImage::from(padded), &config);
        }
        Err(e) => eprintln!("Could not decode image: {}", e),
    }
}

// Formats base stats into a bordered box with color-coded bar graphs.
fn build_stat_lines(stats: &[rustemon::model::pokemon::PokemonStat], total: i64) -> Vec<String> {
    const BAR_WIDTH: usize = 20;
    const INNER: usize = 7 + 1 + 3 + 2 + 1 + BAR_WIDTH + 1; // content width = 35
    let eq   = "=".red().to_string();
    let pipe = "|".red().to_string();
    let sep  = eq.repeat(INNER + 4);

    let mut lines = vec![
        sep.clone(),
        format!("{} {:<width$} {}", pipe, "Base Stats", pipe, width = INNER),
    ];

    for s in stats {
        let name = format_stat_name(&s.stat.name);
        let value: i64 = s.base_stat;
        let bar_len = ((value as f32 / 180.0 * BAR_WIDTH as f32) as usize).min(BAR_WIDTH);
        let bar = colorize_line(&"#".repeat(bar_len), &value);
        let padding = " ".repeat(BAR_WIDTH - bar_len);
        lines.push(format!("{} {:<7} {:>3}  [{}{}] {}", pipe, name, value, bar, padding, pipe));
    }

    lines.push(format!("{} {:<width$} {}", pipe, format!("BST:  {}", total), pipe, width = INNER));
    lines.push(sep);
    lines
}

// Builds the side-by-side info and stat box lines for a Pokémon.
pub async fn pokemon_display_lines(p: &Pokemon, client: &RustemonClient) -> Vec<String> {
    let generation_str = match pokemon_species::get_by_name(&p.species.name, client).await {
        Ok(species) => format_generation(&species.generation.name),
        Err(_) => String::from("Unknown"),
    };

    let height_in_meters = p.height as f32 / 10.0;
    let weight_in_kg = p.weight as f32 / 10.0;

    let abilities_list: String = p
        .abilities
        .iter()
        .filter_map(|a| a.ability.as_ref())
        .map(|ability| format_name(&ability.name))
        .collect::<Vec<_>>()
        .join(", ");

    let formatted_name = format_name(&p.name);
    let types_str = types_to_string(p);
    let types_vis = visible_len(&types_str);

    let value_width = [
        formatted_name.len(),
        format!("{} m", height_in_meters).len(),
        format!("{} kg", weight_in_kg).len(),
        types_vis,
        abilities_list.len(),
    ].iter().copied().max().unwrap_or(10).max(12);

    let info_width = 14 + value_width;
    let eq   = "=".red().to_string();
    let pipe = "|".red().to_string();
    let sep  = eq.repeat(info_width + 1);
    let types_pad = " ".repeat(value_width.saturating_sub(types_vis));

    let info_lines: Vec<String> = vec![
        sep.clone(),
        format!("{} Name:       {:<w$} {}", pipe, formatted_name,             pipe, w = value_width),
        format!("{} Dex No:     {:<w$} {}", pipe, format!("#{}", p.id),       pipe, w = value_width),
        format!("{} Height:     {:<w$} {}", pipe, format!("{} m", height_in_meters), pipe, w = value_width),
        format!("{} Weight:     {:<w$} {}", pipe, format!("{} kg", weight_in_kg),    pipe, w = value_width),
        format!("{} Types:      {}{} {}", pipe, types_str, types_pad,               pipe),
        format!("{} Abilities:  {:<w$} {}", pipe, abilities_list,              pipe, w = value_width),
        format!("{} Generation: {:<w$} {}", pipe, generation_str,              pipe, w = value_width),
        format!("{} {:<w$} {}", pipe, "",                                      pipe, w = value_width + 12),
        sep,
    ];

    let base_stat_total: i64 = p.stats.iter().map(|s| s.base_stat).sum();
    let stat_lines = build_stat_lines(&p.stats, base_stat_total);

    const GAP: usize = 2;
    let max_lines = info_lines.len().max(stat_lines.len());
    let mut lines = Vec::with_capacity(max_lines);

    for i in 0..max_lines {
        let line = match (info_lines.get(i), stat_lines.get(i)) {
            (Some(left), Some(right)) => {
                let pad = " ".repeat(info_width + GAP - visible_len(left));
                format!("{}{}{}", left, pad, right)
            }
            (Some(left), None) => left.clone(),
            (None, Some(right)) => format!("{:>width$}{}", "", right, width = info_width + GAP),
            (None, None) => String::new(),
        };
        lines.push(line);
    }

    lines
}

// Prints the formatted Pokémon info and stat lines to stdout.
pub async fn print_pokemon_info(p: &Pokemon, client: &RustemonClient) {
    for line in pokemon_display_lines(p, client).await {
        println!("{}", line);
    }
}

// Fetches a Pokémon by name, renders its sprite, and prints its info.
pub async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient) -> Result<(), String> {
    let p = pokemon::get_by_name(pokemon_name, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_name, e))?;
    if let Some(url) = p.sprites.front_default.as_deref() {
        if let Some(bytes) = fetch_sprite(url).await {
            display_sprite(bytes);
        }
    }
    print_pokemon_info(&p, client).await;
    let matchup = type_hash(&p, client).await;
    print_type_matchup(&matchup);
    Ok(())
}
