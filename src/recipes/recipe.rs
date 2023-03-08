use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher}, string,
};

use crate::utils::{DynamicDeserialize, FactorioType};

pub enum FactoryType {}

//TODO: Fix 'pub' members in all files.
pub struct Recipe {
    id_hash: u64,
    id: String,
    name: String,
    pub item_ingredients: HashMap<u64, f64>,
    pub fluid_ingredients: HashMap<u64, f64>,
    pub item_products: HashMap<u64, f64>,
    pub fluid_products: HashMap<u64, f64>,
}

impl FactorioType for Recipe {
    fn get_id_hash(&self) -> u64 {
        self.id_hash
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

fn string_to_hash_map(map: &mut HashMap<u64, f64>, input: &String) -> Result<(), String> {
    let split = input.split(',');
    for item in split {
        let item = item.trim();
        let pos = item.find(":");
        if let Some(pos) = pos {
            let (item, amount) = item.split_at(pos);
            let result = amount[1..].trim().parse();
            if let Ok(amount) = result {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                let id = hasher.finish();
                map.insert(id, amount);
            } else {
                return Err(format!("Unable to parse {} as double", amount));
            }
        } else {
            return Err(format!("Expected 'type: value' pair, got {}", item));
        }
    }
    Ok(())
}

impl DynamicDeserialize for Recipe {
    fn deserialize(map: HashMap<String, String>) -> Result<Self, String>
    where
        Self: Sized,
    {
        let id_hash;
        let id;
        let name;
        let mut item_ingredients = HashMap::new();
        let mut fluid_ingredients = HashMap::new();
        let mut item_products = HashMap::new();
        let mut fluid_products = HashMap::new();

        if let Some(raw_id) = map.get("id") {
            id = raw_id.to_string();
            let mut hasher = DefaultHasher::new();
            raw_id.hash(&mut hasher);
            id_hash = hasher.finish();
        } else {
            return Err("Recipe definition is missing 'id' field".to_string());
        }

        if let Some(raw_name) = map.get("name") {
            name = raw_name.to_string();
        } else {
            let mut n = id.clone();
            n[..1].make_ascii_uppercase();
            name = n.replace("-", " ");
        }

        if let Some(input) = map.get("item_ingredients") {
            let result = string_to_hash_map(&mut item_ingredients, input);
            if let Err(message) = result {
                return Err(format!("Unable to parse 'item_ingredients' field for {}: {}", id, message));
            }
        }
        if let Some(input) = map.get("fluid_ingredients") {
            let result = string_to_hash_map(&mut fluid_ingredients, input);
            if let Err(message) = result {
                return Err(format!("Unable to parse 'fluid_ingredients' field for {}: {}", id, message));
            }
        }
        if let Some(input) = map.get("item_products") {
            let result = string_to_hash_map(&mut item_products, input);
            if let Err(message) = result {
                return Err(format!("Unable to parse 'item_products' field for {}: {}", id, message));
            }
        }
        if let Some(input) = map.get("fluid_products") {
            let result = string_to_hash_map(&mut fluid_products, input);
            if let Err(message) = result {
                return Err(format!("Unable to parse 'fluid_products' field for {}: {}", id, message));
            }
        }

        Ok(Recipe {
            id_hash,
            id,
            name,
            item_ingredients,
            fluid_ingredients,
            item_products,
            fluid_products,
        })
    }
}
