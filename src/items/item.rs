use colored::Colorize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    result::Result,
};

use crate::utils::{DynamicDeserialize, FactorioType};

pub struct Item {
    id_hash: u64,
    id: String,
    name: String,
    proto_hash: u64,
    proto: String,
    stack_size: u16,
    icon: String,
}

impl FactorioType for Item {
    fn get_id_hash(&self) -> u64 {
        self.id_hash
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

impl DynamicDeserialize for Item {
    fn deserialize(map: std::collections::HashMap<String, String>) -> Result<Self, String> {
        let id_hash;
        let id;
        let name;
        let proto_hash;
        let proto;
        let stack_size;
        let icon;

        if let Some(raw_id) = map.get("id") {
            id = raw_id.to_string();
            let mut hasher = DefaultHasher::new();
            raw_id.hash(&mut hasher);
            id_hash = hasher.finish();
        } else {
            return Err(format!(
                "{}{}{}",
                "Item definition is missing ".truecolor(255, 195, 63),
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

        if let Some(raw_proto) = map.get("proto") {
            proto = raw_proto.to_string();
            let mut hasher = DefaultHasher::new();
            raw_proto.hash(&mut hasher);
            proto_hash = hasher.finish();
        } else {
            proto_hash = id_hash;
            proto = id.clone();
        }

        if let Some(raw_stack_size) = map.get("stack_size") {
            let result = raw_stack_size.parse();
            if let Ok(size) = result {
                stack_size = size;
            } else {
                return Err(format!(
                    "{}{}{}{}",
                    "Unable to parse ".truecolor(255, 195, 63),
                    "stack_size".bright_yellow(),
                    " field as positive integer for ".truecolor(255, 195, 63),
                    id.bright_yellow()
                ));
            }
        } else {
            return Err(format!(
                "{}{}{}{}{}",
                "Item definition for ".truecolor(255, 195, 63),
                id.bright_yellow(),
                " is missing ".truecolor(255, 195, 63),
                "stack_size".bright_yellow(),
                " field".truecolor(255, 195, 63),
            ));
        }

        if let Some(raw_icon) = map.get("icon") {
            icon = raw_icon.to_string();
        } else {
            icon = format!("{}.png", id);
        }

        Ok(Item {
            id_hash,
            id,
            name,
            proto_hash,
            proto,
            stack_size,
            icon,
        })
    }
}
