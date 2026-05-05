use crate::display::pokemon_display_lines;
use crate::format::{is_transparent, visible_len};
use colored::Colorize;
use image::{ImageBuffer, RgbaImage};
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon;
use std::io::Cursor;

const MAX_CANVAS_W: u32 = 128;

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

    Ok(padded)
}

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
            let gap = 4u32;
            let (a, b) = scale_to_fit(a, b, gap, MAX_CANVAS_W);
            let canvas_w = a.width() + gap + b.width();
            let max_h = a.height().max(b.height());
            let mut canvas: RgbaImage = ImageBuffer::new(canvas_w, max_h);

            image::imageops::overlay(&mut canvas, &a, 0, (max_h - a.height()) as i64);
            image::imageops::overlay(&mut canvas, &b, (a.width() + gap) as i64, (max_h - b.height()) as i64);

            for y in 0..max_h {
                for x in a.width()..(a.width() + gap) {
                    canvas.put_pixel(x, y, image::Rgba([128, 128, 128, 255]));
                }
            }

            let config = viuer::Config {
                transparent: true,
                absolute_offset: false,
                width: Some(canvas_w),
                use_kitty: false,
                use_iterm: false,
                ..Default::default()
            };
            let _ = viuer::print(&image::DynamicImage::from(canvas), &config);
        }
        _ => eprintln!("Could not load one or both sprites."),
    }

    let lines_a = pokemon_display_lines(&pa, client).await;
    let lines_b = pokemon_display_lines(&pb, client).await;

    let col_w = lines_a.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let max_lines = lines_a.len().max(lines_b.len());
    const HALF_GAP: usize = 2;

    for i in 0..max_lines {
        let left = lines_a.get(i).map(|s| s.as_str()).unwrap_or("");
        let right = lines_b.get(i).map(|s| s.as_str()).unwrap_or("");
        let left_pad = " ".repeat(col_w - visible_len(left) + HALF_GAP);
        println!("{}{}{}  {}", left, left_pad, "|".truecolor(180, 70, 0), right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::io::Cursor;

    fn encode_png(img: RgbaImage) -> bytes::Bytes {
        let mut buf = Vec::new();
        DynamicImage::ImageRgba8(img)
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();
        bytes::Bytes::from(buf)
    }

    #[test]
    fn trims_transparent_border_and_adds_padding() {
        // 10x10 all-transparent except pixel (5,5)
        let mut img = RgbaImage::new(10, 10);
        img.put_pixel(5, 5, Rgba([255, 0, 0, 255]));
        let result = format_sprite(encode_png(img)).unwrap();
        // trimmed core is 1x1; padded by 4 on each side → 9x9
        assert_eq!(result.width(), 9);
        assert_eq!(result.height(), 9);
    }

    #[test]
    fn does_not_scale_down_large_image() {
        // 100x100 fully opaque → no transparent border → padded to 108x108, no further scaling
        let img = RgbaImage::from_pixel(100, 100, Rgba([0, 200, 0, 255]));
        let result = format_sprite(encode_png(img)).unwrap();
        assert_eq!(result.width(), 108);
        assert_eq!(result.height(), 108);
    }

    #[test]
    fn preserves_size_for_small_image() {
        // 20x20 opaque → padded 28x28, no scaling needed
        let img = RgbaImage::from_pixel(20, 20, Rgba([0, 0, 255, 255]));
        let result = format_sprite(encode_png(img)).unwrap();
        assert_eq!(result.width(), 28);
        assert_eq!(result.height(), 28);
    }

    #[test]
    fn returns_error_for_invalid_bytes() {
        let bad = bytes::Bytes::from("not a png");
        assert!(format_sprite(bad).is_err());
    }

    #[test]
    fn scale_to_fit_caps_oversized_canvas() {
        let a = RgbaImage::from_pixel(100, 100, Rgba([255, 0, 0, 255]));
        let b = RgbaImage::from_pixel(100, 100, Rgba([0, 0, 255, 255]));
        let (sa, sb) = scale_to_fit(a, b, 4, 128);
        assert!(sa.width() + 4 + sb.width() <= 128);
    }

    #[test]
    fn scale_to_fit_preserves_small_sprites() {
        let a = RgbaImage::from_pixel(60, 60, Rgba([255, 0, 0, 255]));
        let b = RgbaImage::from_pixel(60, 60, Rgba([0, 0, 255, 255]));
        let (sa, sb) = scale_to_fit(a, b, 4, 128);
        // 60+4+60=124 ≤ 128, no scaling
        assert_eq!(sa.width(), 60);
        assert_eq!(sb.width(), 60);
    }
}
