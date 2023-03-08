use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

use crate::utils::FactorioType;

#[derive(Deserialize)]
pub struct SerializableItem {
    id: String,
    name: String,
    prototype: String,
    stack_size: u16,
    is_fluid: bool,
}

pub struct Item {
    hash_id: u64,
    id: String,
    name: String,
    prototype: u64,
    stack_size: u16,
    is_fluid: bool,
    icon: String,
}

impl Item {
    pub fn new(base: SerializableItem) -> Item {
        let mut hasher = DefaultHasher::new();
        base.id.hash(&mut hasher);
        let hash_id = hasher.finish();
        let mut hasher = DefaultHasher::new();
        base.prototype.hash(&mut hasher);
        let proto = hasher.finish();
        let icon = format!("{}.png", base.id.clone());
        Item {
            hash_id,
            id: base.id,
            name: base.name,
            prototype: proto,
            stack_size: base.stack_size,
            is_fluid: base.is_fluid,
            icon: icon,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{:#018x}: {}", self.hash_id, self.id))?;
        //f.write_str(self.name.as_str())?;
        return Result::Ok(());
    }
}
