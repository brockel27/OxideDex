use crate::format::*;
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon;
use std::io::Cursor;

//const LOGO_BYTES: &[u8] = include_bytes!("../OxideDex_logo.png");

/*pub async fn display_logo() {
    display_sprite(bytes::Bytes::from_static(LOGO_BYTES));
}*/

async fn fetch_sprite(url: &str) -> Option<bytes::Bytes> {
    match reqwest::get(url).await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => Some(bytes),
            Err(e) => { eprintln!("Could not download sprite: {}", e); None }
        },
        Err(e) => { eprintln!("Could not fetch sprite: {}", e); None }
    }
}

fn display_sprite(bytes: bytes::Bytes) {
    match image::load(Cursor::new(bytes), image::ImageFormat::Png) {
        Ok(img) => {
            let (mut top, mut bottom) = (0, img.height() - 1);
            let (mut left, mut right) = (0, img.width() - 1);
            while top < bottom && is_transparent(&img, top, true) { top += 1; }
            while bottom > top && is_transparent(&img, bottom, true) { bottom -= 1; }
            while left < right && is_transparent(&img, left, false) { left += 1; }
            while right > left && is_transparent(&img, right, false) { right -= 1; }
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
                width: Some(96),
                use_kitty: false,
                use_iterm: false,
                ..Default::default()
            };
            let _ = viuer::print(&image::DynamicImage::from(padded), &config);
        }
        Err(e) => eprintln!("Could not decode image: {}", e),
    }
}

// ANSI escape bytes in colored strings would inflate a naive `.len()`.
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' { in_escape = true; }
        else if in_escape { if c == 'm' { in_escape = false; } }
        else { len += 1; }
    }
    len
}

fn build_stat_lines(stats: &[rustemon::model::pokemon::PokemonStat], total: i64) -> Vec<String> {
    const BAR_WIDTH: usize = 20;
    const INNER: usize = 7 + 1 + 3 + 2 + 1 + BAR_WIDTH + 1; // content width = 35
    let sep = "=".repeat(INNER + 3);
    let mut lines = vec![
        sep.clone(),
        format!("| {:<width$}|", "Base Stats", width = INNER),
    ];
    for s in stats {
        let name = format_stat_name(&s.stat.name);
        let value = s.base_stat;
        let bar_len = ((value as f32 / 180.0 * BAR_WIDTH as f32) as usize).min(BAR_WIDTH);
        let bar = "#".repeat(bar_len);
        lines.push(format!("| {:<7} {:>3}  [{:<bw$}]|", name, value, bar, bw = BAR_WIDTH));
    }
    lines.push(format!("| {:<width$}|", format!("BST:  {}", total), width = INNER));
    lines.push(sep);
    lines
}

pub async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient) {
    match pokemon::get_by_name(pokemon_name, client).await {
        Ok(p) => {
            if let Some(url) = p.sprites.front_default.as_deref() {
                if let Some(bytes) = fetch_sprite(url).await {
                    display_sprite(bytes);
                }
            }

            // Height originally in decimeters (dm), Weight in hectograms (hg)
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
            let types_str = types_to_string(&p);
            let types_vis = visible_len(&types_str);

            let value_width = [
                formatted_name.len(),
                format!("{} m", height_in_meters).len(),
                format!("{} kg", weight_in_kg).len(),
                types_vis,
                abilities_list.len(),
            ].iter().copied().max().unwrap_or(10).max(12);

            let info_width = 14 + value_width;
            let sep = "=".repeat(info_width);
            let types_pad = " ".repeat(value_width.saturating_sub(types_vis));

            let info_lines: Vec<String> = vec![
                sep.clone(),
                format!("| Name:      {:<w$}|", formatted_name,             w = value_width),
                format!("| Height:    {:<w$}|", format!("{} m", height_in_meters), w = value_width),
                format!("| Weight:    {:<w$}|", format!("{} kg", weight_in_kg),    w = value_width),
                format!("| Types:     {}{}|", types_str, types_pad),
                format!("| Abilities: {:<w$}|", abilities_list,              w = value_width),
                sep,
            ];

            let base_stat_total: i64 = p.stats.iter().map(|s| s.base_stat).sum();
            let stat_lines = build_stat_lines(&p.stats, base_stat_total);

            // Two-column layout
            const GAP: usize = 2;
            let max_lines = info_lines.len().max(stat_lines.len());
            for i in 0..max_lines {
                match (info_lines.get(i), stat_lines.get(i)) {
                    (Some(left), Some(right)) => {
                        let pad = " ".repeat(info_width + GAP - visible_len(left));
                        println!("{}{}{}", left, pad, right);
                    }
                    (Some(left), None) => println!("{}", left),
                    (None, Some(right)) => {
                        println!("{:>width$}{}", "", right, width = info_width + GAP);
                    }
                    (None, None) => {}
                }
            }
        }

        Err(error) => {
            eprintln!("Error: Could not find '{}'. ({})", pokemon_name, error);
        }
    }
}
