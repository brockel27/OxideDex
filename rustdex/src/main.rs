use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::pokemon::pokemon;
use std::env;

async fn display_pokemon_data(pokemon_name: &str, client: &RustemonClient) {
    // Rustemon's get_by_name handles the API request and JSON parsing
    match pokemon::get_by_name(pokemon_name, client).await {
        Ok(p) => {
            // Convert the integers from the API to floats
            // Height orignally in decimeters (dm), Weight in hectograms (hg)
            let height_in_meters = p.height as f32 / 10.0;
            let weight_in_kg = p.weight as f32 / 10.0;

            // Map the abilities into a readable String
            let abilities_list: String = p.abilities
                .iter()
                .map(|a| a.ability.as_ref().unwrap().name.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            println!("Name: {}", p.name);
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

fn types_to_string(pokemon: &Pokemon) -> String {
    pokemon
        .types
        .iter()
        .map(|ptype| ptype.type_.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[tokio::main]
async fn main() {
    // std::env::args() doen't allow access to uninitialized memory
    let args: Vec<String> = env::args().collect();

    // Initialize the client
    let client = RustemonClient::default();

    if args.len() == 2 {
        let pokemon_name = args[1].to_lowercase();
        display_pokemon_data(&pokemon_name, &client).await;
    } else {
        eprintln!("Usage: rustdex <pokemon_name_or_id>");
    }
}
