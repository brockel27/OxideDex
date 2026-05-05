use std::collections::HashMap;
use crate::RustemonClient;
use rustemon::model::pokemon::Pokemon;

const TYPE_NAMES: [&str; 18] = [
    "normal", "fighting", "flying", "poison", "ground",
    "rock", "bug", "ghost", "steel", "fire",
    "water", "grass", "electric", "psychic", "ice",
    "dragon", "dark", "fairy",
];

// the first thing is to declare a new hashmap:
pub async fn type_hash(p: &Pokemon, client: &RustemonClient) -> HashMap<String, f32> {
    let mut p_types = HashMap::new();

    for items in TYPE_NAMES {
        p_types.insert(items.to_string(), 1.0);
    }

    for type_slot in &p.types {
        let type_data = rustemon::pokemon::type_::get_by_name(type_slot.type_.name.as_str(), client).await.unwrap();
        type_data.damage_relations;
    }
    p_types
}

/*
damage_relations includes:

double_damage_from (weak to)
double_damage_to (strong against)
half_damage_from (resists)
no_damage_from (immune to taking damage from this type)
no_damage_to (immune to dealing damage TO this type) */
