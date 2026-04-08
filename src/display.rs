use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::pokemon::pokemon;
use colored::*;
use image::{DynamicImage, GenericImageView};
use std::io::Cursor;

// Helper to apply colors based on the type name
pub fn colorize_type(type_name: &str) -> ColoredString {
    let formatted = format_name(type_name);
    match type_name.to_lowercase().as_str() {
        "fire" => formatted.red().bold(),
        "water" => formatted.blue().bold(),
        "grass" => formatted.green().bold(),
        "electric" => formatted.yellow().bold(),
        "ice" => formatted.cyan().bold(),
        "poison" => formatted.magenta().bold(),
        "fighting" => formatted.truecolor(255, 128, 0).bold(),
        "ground" => formatted.truecolor(226, 191, 101).bold(),
        "flying" => formatted.truecolor(184, 143, 243).bold(),
        "psychic" => formatted.truecolor(249, 85, 135).bold(),
        "bug" => formatted.truecolor(166, 185, 26).bold(),
        "rock" => formatted.truecolor(182, 161, 54).bold(),
        "ghost" => formatted.truecolor(115, 87, 151).bold(),
        "dragon" => formatted.truecolor(111, 53, 252).bold(),
        "dark" => formatted.truecolor(112, 87, 70).bold(),
        "steel" => formatted.truecolor(183, 183, 206).bold(),
        "fairy" => formatted.truecolor(214, 133, 173).bold(),
        _ => formatted.normal(),
    }
}

pub fn format_name(name: &str) -> String {
    name.split('-') // Split at hyphens
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                // Capitalize first letter, keep the rest as is
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ") // Join back with spaces
}

pub fn types_to_string(p: &Pokemon) -> String {
    p.types
        .iter()
        .map(|ptype| {
            let name = &ptype.type_.name;
            // Format the name first (e.g., "fire"), then colorize it
            colorize_type(name).to_string()
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn is_transparent(img: &DynamicImage, start: u32, is_row: bool) -> bool {
    if is_row {
        (0..img.width()).all(|x| img.get_pixel(x, start).0[3] == 0)
    } else {
        (0..img.height()).all(|y| img.get_pixel(start, y).0[3] == 0)
    }
}

async fn fetch_and_display_sprite(pokemon_id: i64) {
    let url = format!(
        "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png",
        pokemon_id
    );
    // fetch headers of image data stream
    match reqwest::get(&url).await {
        // fetch actual bytes in body of image data stream
        Ok(response) => match response.bytes().await {
            Ok(bytes) => {
                match image::load(Cursor::new(bytes), image::ImageFormat::Png) {
                    Ok(img) => {
                        let (mut top, mut bottom) = (0, img.height() - 1);
                        let (mut left, mut right) = (0, img.width() - 1);

                        while top < bottom && is_transparent(&img, top, true) { top += 1; }
                        while bottom > top && is_transparent(&img, bottom, true) { bottom -= 1; }
                        while left < right && is_transparent(&img, left, false) { left += 1; }
                        while right > left && is_transparent(&img, right, false) { right -= 1; }

                        let trimmed = img.crop_imm(left, top, right - left + 1, bottom - top + 1);

                        let config = viuer::Config {
                            transparent: true,
                            absolute_offset: false,
                            width: Some(48),
                            ..Default::default()
                        };
                        let _ = viuer::print(&trimmed, &config);
                    }
                    Err(e) => eprintln!("Could not decode sprite: {}", e),
                }
            }
            Err(e) => eprintln!("Could not download sprite: {}", e),
        },
        Err(e) => eprintln!("Could not fetch sprite: {}", e),
    }
}

pub async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient) {
    // Rustemon's get_by_name handles the API request and JSON parsing
    match pokemon::get_by_name(pokemon_name, client).await {
        Ok(p) => {

            // p.sprites is a rustemon Sprites struct
            // The front_default field is Option<String>
  //          if let Some(p.id) = &p.sprites.front_default {
                fetch_and_display_sprite(p.id).await;
    //        }

            // Convert the integers from the API to floats
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

            println!("Name: {}", formatted_name);
            println!("Height: {} m", height_in_meters);
            println!("Weight: {} kg", weight_in_kg);
            println!("Types: {}", types_to_string(&p));
            println!("Abilities: {}", abilities_list);
        }

        Err(error) => {
            eprintln!("Error: Could not find '{}'. ({})", pokemon_name, error);
        }
    }
}
