mod pokemon;

use pokemon::{fetch_pokemon_data, Pokemon};
use std::env;
use tokio;

async fn display_pokemon_data(pokemon_name: &str) {
   match fetch_pokemon_data(pokemon_name).await {
      Ok(pokemon) => {
         println!("Name: {}", pokemon.name);
         println!("Height: {}", pokemon.height);
         println!("Weight: {}", types_to_string(&pokemon.types));
         println!("Types: {}", types_to_string(&pokemon.types));
      }
      Err(error) => {
         eprintln!("Error: {}", error);
      }
   }
}

fn types_to_string(types: &[pokemon::PokemonType]) -> String {
   types
      .iter()
      .map(|ptype| ptype.type_data.name.as_str())
      .collect::<Vec<_>>()
      .join(", ")
}

#[tokio::main]
async fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() == 2 {
      let pokemon_name = &args[1];
      display_pokemon_data(pokemon_name).await;
   } else {
      eprintln!("Usage: pokeapi_rust <pokemon_name>");
   }
}


