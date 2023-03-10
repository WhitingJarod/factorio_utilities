use colored::Colorize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    result::Result,
};

use crate::utils::{DynamicDeserialize, FactorioType};

pub struct Fluid {
    pub id_hash: u64,
    pub id: String,
    pub name: String,
    pub proto_hash: u64,
    pub proto: String,
    pub icon: String,
}

impl FactorioType for Fluid {
    fn get_id_hash(&self) -> u64 {
        self.id_hash
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

impl DynamicDeserialize for Fluid {
    fn deserialize(map: std::collections::HashMap<String, String>) -> Result<Self, String> {
        let id_hash;
        let id;
        let name;
        let proto_hash;
        let proto;
        let icon;

        if let Some(raw_id) = map.get("id") {
            id = raw_id.to_string();
            let mut hasher = DefaultHasher::new();
            raw_id.hash(&mut hasher);
            id_hash = hasher.finish();
        } else {
            return Err(format!(
                "{}{}{}",
                "Fluid definition is missing ".truecolor(255, 195, 63),
                "id".bright_yellow(),
                " field".truecolor(255, 195, 63)
            ));
        }

        if let Some(raw_name) = map.get("name") {
            name = raw_name.to_string();
        } else {
            let mut n = id.clone();
            n[..1].make_ascii_uppercase();
            name = n.replace("-", " ");
        }

        proto_hash = 0xde26367c3e8bff21;
        proto = "fluid".to_string();

        if let Some(raw_icon) = map.get("icon") {
            icon = raw_icon.to_string();
        } else {
            icon = format!("{}.png", id);
        }

        Ok(Fluid {
            id_hash,
            id,
            name,
            proto_hash,
            proto,
            icon,
        })
    }
}
