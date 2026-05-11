use crate::display::{fetch_sprite, get_flavor_text, pokemon_display_lines, wrap_text};
use crate::format::{border_bottom, border_row, border_top, is_col_transparent, is_row_transparent, visible_len};
use crossterm::terminal;
use crate::type_matchup::type_hash;
use crate::type_matchup::build_type_matchup_lines;

use colored::Colorize;
use image::{ImageBuffer, RgbaImage};
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon;
use std::io::Cursor;

const MAX_CANVAS_W: u32 = 128;

// Decodes and trims transparent border from a PNG sprite.
fn format_sprite(bytes: bytes::Bytes) -> Result<RgbaImage, image::ImageError> {
    let img = image::load(Cursor::new(bytes), image::ImageFormat::Png)?;

    let (mut top, mut bottom) = (0, img.height() - 1);
    let (mut left, mut right) = (0, img.width() - 1);
    while top < bottom && is_row_transparent(&img, top) { top += 1; }
    while bottom > top && is_row_transparent(&img, bottom) { bottom -= 1; }
    while left < right && is_col_transparent(&img, left) { left += 1; }
    while right > left && is_col_transparent(&img, right) { right -= 1; }

    Ok(img.crop_imm(left, top, right - left + 1, bottom - top + 1).to_rgba8())
}

// Proportionally downscales both sprites so their combined width fits within max_w.
fn scale_to_fit(a: RgbaImage, b: RgbaImage, gap: u32, max_w: u32) -> (RgbaImage, RgbaImage) {
    let canvas_w = a.width() + gap + b.width();
    if canvas_w <= max_w {
        return (a, b);
    }
    let scale = max_w as f32 / canvas_w as f32;
    let scaled = |img: RgbaImage| -> RgbaImage {
        let nw = ((img.width() as f32 * scale) as u32).max(1);
        let nh = ((img.height() as f32 * scale) as u32).max(1);
        image::imageops::resize(&img, nw, nh, image::imageops::FilterType::Lanczos3)
    };
    (scaled(a), scaled(b))
}

// Fetches and processes a sprite from a URL, returning None on any error.
async fn load_sprite(url: &str) -> Option<RgbaImage> {
    fetch_sprite(url).await.and_then(|b| format_sprite(b).ok())
}

// Composites two sprites side-by-side with a divider strip and renders the result centered within text_width columns.
fn render_composite(a: RgbaImage, b: RgbaImage, text_width: usize) {
    const GAP: u32 = 4;
    let (a, b) = scale_to_fit(a, b, GAP, MAX_CANVAS_W);
    let canvas_w = a.width() + GAP + b.width();
    let max_h = a.height().max(b.height());
    let mut canvas: RgbaImage = ImageBuffer::new(canvas_w, max_h);

    image::imageops::overlay(&mut canvas, &a, 0, (max_h - a.height()) as i64);
    image::imageops::overlay(&mut canvas, &b, (a.width() + GAP) as i64, (max_h - b.height()) as i64);

    let x_offset = (text_width as u32).saturating_sub(canvas_w) / 2;
    let config = viuer::Config {
        transparent: true,
        absolute_offset: false,
        x: x_offset as u16,
        width: Some(canvas_w),
        use_kitty: false,
        use_iterm: false,
        ..Default::default()
    };
    let _ = viuer::print(&image::DynamicImage::from(canvas), &config);
}

// Fetches two Pokémon, renders their sprites side-by-side, and prints their info and type matchup boxes.
pub async fn display_dual(pokemon_a: &str, pokemon_b: &str, client: &RustemonClient, shiny_a: bool, shiny_b: bool) -> Result<(), String> {
    let pa = pokemon::get_by_name(pokemon_a, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_a, e))?;
    let pb = pokemon::get_by_name(pokemon_b, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_b, e))?;

    // Cap each column so the combined display fits within the terminal width.
    // total_width = 2*col_w + 9, col_w = 55 + value_width => max_value_width = (term_w - 119) / 2
    let term_w = terminal::size().map(|(w, _)| w as usize).unwrap_or(200);
    let max_value_width = ((term_w.saturating_sub(119)) / 2).max(12);

    let display_lines_a = pokemon_display_lines(&pa, client, Some(max_value_width)).await;
    let display_lines_b = pokemon_display_lines(&pb, client, Some(max_value_width)).await;

    const COL_GAP: usize = 2;
    let col_w_a = display_lines_a.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let col_w_b = display_lines_b.iter().map(|l| visible_len(l)).max().unwrap_or(0);

    let matchup_lines_a = build_type_matchup_lines(&type_hash(&pa, client).await, col_w_a);
    let matchup_lines_b = build_type_matchup_lines(&type_hash(&pb, client).await, col_w_b);
    let text_width = col_w_a + COL_GAP + 1 + 2 + col_w_b;

    let url_a = if shiny_a { pa.sprites.front_shiny.as_deref() } else { pa.sprites.front_default.as_deref() };
    let url_b = if shiny_b { pb.sprites.front_shiny.as_deref() } else { pb.sprites.front_default.as_deref() };
    let sprite_a = match url_a { Some(url) => load_sprite(url).await, None => None };
    let sprite_b = match url_b { Some(url) => load_sprite(url).await, None => None };

    println!("{}", border_top(text_width));
    match (sprite_a, sprite_b) {
        // text_width + 4 = outer border width, so composite centers within the bordered frame
        (Some(a), Some(b)) => render_composite(a, b, text_width + 4),
        _ => eprintln!("Could not load one or both sprites."),
    }

    let max_info = display_lines_a.len().max(display_lines_b.len());
    for i in 0..max_info {
        let left  = display_lines_a.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = display_lines_b.get(i).map(|s| s.as_str()).unwrap_or("");
        let pad = " ".repeat(col_w_a.saturating_sub(visible_len(left)) + COL_GAP);
        let line = format!("{}{}{}  {}", left, pad, "|".truecolor(120, 115, 110), right);
        println!("{}", border_row(&line, text_width));
    }

    let max_matchup = matchup_lines_a.len().max(matchup_lines_b.len());
    for i in 0..max_matchup {
        let left  = matchup_lines_a.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = matchup_lines_b.get(i).map(|s| s.as_str()).unwrap_or("");
        let pad = " ".repeat(col_w_a.saturating_sub(visible_len(left)) + COL_GAP);
        let line = format!("{}{}{}  {}", left, pad, "|".truecolor(120, 115, 110), right);
        println!("{}", border_row(&line, text_width));
    }

    let flavor_a = get_flavor_text(&pa.species.name, client).await;
    let flavor_b = get_flavor_text(&pb.species.name, client).await;

    if flavor_a.is_some() || flavor_b.is_some() {
        const BCLR: (u8, u8, u8) = (225, 170, 160);
        let eq  = "=".truecolor(BCLR.0, BCLR.1, BCLR.2).to_string();
        let sep = eq.repeat(text_width);
        println!("{}", border_row(&sep, text_width));

        let label     = "Pokédex:".truecolor(200, 200, 200).bold().to_string();
        let label_vis = visible_len(&label);
        let indent    = " ".repeat(label_vis + 1);
        let wrap_w_a  = col_w_a.saturating_sub(label_vis + 2);
        let wrap_w_b  = col_w_b.saturating_sub(label_vis + 2);

        let wrapped_a = flavor_a.as_deref().map(|t| wrap_text(t, wrap_w_a)).unwrap_or_default();
        let wrapped_b = flavor_b.as_deref().map(|t| wrap_text(t, wrap_w_b)).unwrap_or_default();

        let max_ft = wrapped_a.len().max(wrapped_b.len());
        for i in 0..max_ft {
            let prefix_a = if i == 0 { format!("{} ", label) } else { indent.clone() };
            let prefix_b = if i == 0 { format!("{} ", label) } else { indent.clone() };
            let left  = format!("{}{}", prefix_a, wrapped_a.get(i).map(|s| s.as_str()).unwrap_or(""));
            let right = format!("{}{}", prefix_b, wrapped_b.get(i).map(|s| s.as_str()).unwrap_or(""));
            let pad   = " ".repeat(col_w_a.saturating_sub(visible_len(&left)) + COL_GAP);
            let line  = format!("{}{}{}  {}", left, pad, "|".truecolor(120, 115, 110), right);
            println!("{}", border_row(&line, text_width));
        }
    }

    println!("{}", border_bottom(text_width));
    Ok(())
}
