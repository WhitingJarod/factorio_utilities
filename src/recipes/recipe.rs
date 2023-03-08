use serde::Deserialize;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::utils::FactorioType;

#[derive(Deserialize)]
struct SerializableRecipe {
    id: String,
    name: String,
    ingredients: HashMap<String, f64>,
    products: HashMap<String, f64>,
}

struct Recipe {
    hash_id: u64,
    id: String,
    name: String,
    ingredients: HashMap<u64, f64>,
    products: HashMap<u64, f64>,
}

impl Recipe {
    pub fn new(base: SerializableRecipe) -> Recipe {
        let mut hasher = DefaultHasher::new();
        base.id.hash(&mut hasher);
        let hash_id = hasher.finish();

        let mut ingredients = HashMap::new();
        for key in base.ingredients.keys() {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let hash_key = hasher.finish();
            ingredients.insert(hash_key, *base.ingredients.get(key).unwrap());
        }

        let mut products = HashMap::new();
        for key in base.products.keys() {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let hash_key = hasher.finish();
            products.insert(hash_key, *base.products.get(key).unwrap());
        }

        Recipe {
            hash_id,
            id: base.id,
            name: base.name,
            ingredients,
            products
        }
    }
}
