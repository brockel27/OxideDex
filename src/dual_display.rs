use crate::display::print_pokemon_info;
use crate::format::is_transparent;
use image::{ImageBuffer, RgbaImage};
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon;
use std::io::Cursor;

const SPRITE_MAX: u32 = 72;

fn format_sprite(bytes: bytes::Bytes) -> Result<RgbaImage, image::ImageError> {
    let img = image::load(Cursor::new(bytes), image::ImageFormat::Png)?;

    let (mut top, mut bottom) = (0, img.height() - 1);
    let (mut left, mut right) = (0, img.width() - 1);
    while top < bottom && is_transparent(&img, top, true) { top += 1; }
    while bottom > top && is_transparent(&img, bottom, true) { bottom -= 1; }
    while left < right && is_transparent(&img, left, false) { left += 1; }
    while right > left && is_transparent(&img, right, false) { right -= 1; }
    let trimmed = img.crop_imm(left, top, right - left + 1, bottom - top + 1);

    let pad = 4u32;
    let trimmed_rgba = trimmed.to_rgba8();
    let mut padded = RgbaImage::new(
        trimmed_rgba.width() + pad * 2,
        trimmed_rgba.height() + pad * 2,
    );
    image::imageops::overlay(&mut padded, &trimmed_rgba, pad as i64, pad as i64);

    let (w, h) = (padded.width(), padded.height());
    if w > SPRITE_MAX || h > SPRITE_MAX {
        let scale = SPRITE_MAX as f32 / w.max(h) as f32;
        let (nw, nh) = ((w as f32 * scale) as u32, (h as f32 * scale) as u32);
        Ok(image::imageops::resize(&padded, nw, nh, image::imageops::FilterType::Lanczos3))
    } else {
        Ok(padded)
    }
}

async fn fetch_sprite(url: &str) -> Option<bytes::Bytes> {
    match reqwest::get(url).await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => Some(bytes),
            Err(e) => { eprintln!("Could not download sprite: {}", e); None }
        },
        Err(e) => { eprintln!("Could not fetch sprite: {}", e); None }
    }
}

pub async fn display_dual(pokemon_a: &str, pokemon_b: &str, client: &RustemonClient) {
    let (pa, pb) = match (
        pokemon::get_by_name(pokemon_a, client).await,
        pokemon::get_by_name(pokemon_b, client).await,
    ) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(e), _) => { eprintln!("Error: Could not find '{}'. ({})", pokemon_a, e); return; }
        (_, Err(e)) => { eprintln!("Error: Could not find '{}'. ({})", pokemon_b, e); return; }
    };

    let sprite_a: Option<RgbaImage> = match pa.sprites.front_default.as_deref() {
        Some(url) => fetch_sprite(url).await.and_then(|b| format_sprite(b).ok()),
        None => None,
    };

    let sprite_b: Option<RgbaImage> = match pb.sprites.front_default.as_deref() {
        Some(url) => fetch_sprite(url).await.and_then(|b| format_sprite(b).ok()),
        None => None,
    };

    match (sprite_a, sprite_b) {
        (Some(a), Some(b)) => {
            let max_h = a.height().max(b.height());
            let mut canvas: RgbaImage = ImageBuffer::new(164, max_h);

            image::imageops::overlay(&mut canvas, &a, 0, (max_h - a.height()) as i64);
            image::imageops::overlay(&mut canvas, &b, 84, (max_h - b.height()) as i64);

            for y in 0..max_h {
                for x in 80..84 {
                    canvas.put_pixel(x, y, image::Rgba([128, 128, 128, 255]));
                }
            }

            let config = viuer::Config {
                transparent: true,
                absolute_offset: false,
                width: Some(160),
                use_kitty: false,
                use_iterm: false,
                ..Default::default()
            };
            let _ = viuer::print(&image::DynamicImage::from(canvas), &config);
        }
        _ => eprintln!("Could not load one or both sprites."),
    }

    print_pokemon_info(&pa, client).await;
    print_pokemon_info(&pb, client).await;
}
