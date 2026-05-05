use crate::display::{fetch_sprite, pokemon_display_lines};
use crate::format::{is_col_transparent, is_row_transparent, visible_len};
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
    const GAP: u32 = 2;
    let (a, b) = scale_to_fit(a, b, GAP, MAX_CANVAS_W);
    let canvas_w = a.width() + GAP + b.width();
    let max_h = a.height().max(b.height());
    let mut canvas: RgbaImage = ImageBuffer::new(canvas_w, max_h);

    image::imageops::overlay(&mut canvas, &a, 0, (max_h - a.height()) as i64);
    image::imageops::overlay(&mut canvas, &b, (a.width() + GAP) as i64, (max_h - b.height()) as i64);

    for y in 0..max_h {
        for x in a.width()..(a.width() + GAP) {
            canvas.put_pixel(x, y, image::Rgba([180, 70, 0, 255]));
        }
    }

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
pub async fn display_dual(pokemon_a: &str, pokemon_b: &str, client: &RustemonClient) -> Result<(), String> {
    let pa = pokemon::get_by_name(pokemon_a, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_a, e))?;
    let pb = pokemon::get_by_name(pokemon_b, client).await
        .map_err(|e| format!("Could not find '{}'. ({})", pokemon_b, e))?;

    let display_lines_a = pokemon_display_lines(&pa, client).await;
    let display_lines_b = pokemon_display_lines(&pb, client).await;

    const COL_GAP: usize = 2;
    let col_w_a = display_lines_a.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let col_w_b = display_lines_b.iter().map(|l| visible_len(l)).max().unwrap_or(0);

    let matchup_lines_a = build_type_matchup_lines(&type_hash(&pa, client).await, col_w_a);
    let matchup_lines_b = build_type_matchup_lines(&type_hash(&pb, client).await, col_w_b);
    let text_width = col_w_a + COL_GAP + 1 + 2 + col_w_b;

    let sprite_a = match pa.sprites.front_default.as_deref() {
        Some(url) => load_sprite(url).await,
        None => None,
    };
    let sprite_b = match pb.sprites.front_default.as_deref() {
        Some(url) => load_sprite(url).await,
        None => None,
    };

    match (sprite_a, sprite_b) {
        (Some(a), Some(b)) => render_composite(a, b, text_width),
        _ => eprintln!("Could not load one or both sprites."),
    }

    let max_info = display_lines_a.len().max(display_lines_b.len());
    for i in 0..max_info {
        let left  = display_lines_a.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = display_lines_b.get(i).map(|s| s.as_str()).unwrap_or("");
        let pad = " ".repeat(col_w_a.saturating_sub(visible_len(left)) + COL_GAP);
        println!("{}{}{}  {}", left, pad, "|".truecolor(180, 70, 0), right);
    }

    let max_matchup = matchup_lines_a.len().max(matchup_lines_b.len());
    for i in 0..max_matchup {
        let left  = matchup_lines_a.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = matchup_lines_b.get(i).map(|s| s.as_str()).unwrap_or("");
        let pad = " ".repeat(col_w_a.saturating_sub(visible_len(left)) + COL_GAP);
        println!("{}{}{}  {}", left, pad, "|".truecolor(180, 70, 0), right);
    }
    Ok(())
}
