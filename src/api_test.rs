use serde::Deserialize;

/* These structs will help us deserialize the
   JSON data returned from the PokeAPI */

#[derive(Deserialize, Debug)]
pub struct Pokemon {
   pub name: String,
   pub height: u32,
   pub weight: u32,
   pub types: Vec<TypeEntry>,
}

#[derive(Deserialize, Debug)]
pub struct TypeEntry {
   #[serde(rename = "type")]
   pub type_data: TypeData,
}

#[derive(Deserialize, Debug)]
pub struct TypeData {
   pub name: String,
}


/* The fetch_pokemon_data function is an asynchronous function
that takes a reference to a pokemon_name string and returns
a Result with either a Pokemon struct or an Error.In the
body of the function, we format the PokeAPI URL and make
an HTTP GET request using reqwest::get. Then, we use the
json method to deserialize the response into a Pokemon struct. */


//  tldr: A function that fetches a Pokémon’s data from the PokeAPI

use reqwest::Error;

pub async fn fetch_pokemon_data(pokemon_name: &str) -> Result<Pokemon, Error> {
   let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
   let response = reqwest::get(&url).await?;
   let pokemon: Pokemon = response.json().await?;
   Ok(pokemon)
}
