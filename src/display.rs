use crate::format::*;
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon;
use std::io::Cursor;

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

            // Re-add a small transparent border so bilinear resize doesn't blend
            // edge content pixels with the image boundary, causing fringe artifacts.
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
        Err(e) => eprintln!("Could not decode sprite: {}", e),
    }
}

pub async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient) {
    // Rustemon's get_by_name handles the API request and JSON parsing
    match pokemon::get_by_name(pokemon_name, client).await {
        Ok(p) => {

            if let Some(url) = p.sprites.front_default.as_deref() {
                if let Some(bytes) = fetch_sprite(url).await {
                    display_sprite(bytes);
                }
            }

            // Height orignally in decimeters (dm), Weight in hectograms (hg)
            let height_in_meters = p.height as f32 / 10.0;
            let weight_in_kg = p.weight as f32 / 10.0;

            // Map the abilities into a readable String, originally a Vec
            let abilities_list: String = p
                .abilities
                .iter()
                .filter_map(|a| a.ability.as_ref())
                .map(|ability| format_name(&ability.name))
                .collect::<Vec<_>>()
                .join(", ");

            let formatted_name = format_name(&p.name);

            println!("============");
            println!("Name: {}", formatted_name);
            println!("Height: {} m", height_in_meters);
            println!("Weight: {} kg", weight_in_kg);
            println!("Types: {}", types_to_string(&p));
            println!("Abilities: {}", abilities_list);
            println!("");
            println!("Base Stats:");
            println!("============");            

            let bar_width = 50.0;

            p.stats.iter().for_each(|s| {
                print!("{}: {}", format_stat_name(&s.stat.name), s.base_stat);
                print!("   [");

                let bar_length = (s.base_stat as f32 / 255.0 * bar_width as f32) as usize;

                for _element in 1..=bar_length {
                    print!("#");
                }
                println!("]");
            });

            let base_stat_total: i64 = p.stats.iter().map(|s| s.base_stat).sum();

            println!("___");
            println!("BST: {}", base_stat_total);
        }

        Err(error) => {
            eprintln!("Error: Could not find '{}'. ({})", pokemon_name, error);
        }
    }
}
