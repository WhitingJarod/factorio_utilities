use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct SerializableItem {
    pub id: String,
    pub name: String,
    pub prototype: String,
    pub stack_size: u16,
    pub is_fluid: bool,
}

impl SerializableItem {
    pub fn to_item(self) -> Item {
        Item::new(self)
    }
}

pub struct Item {
    pub uuid: u64,
    pub id: String,
    pub name: String,
    pub prototype: u64,
    pub stack_size: u16,
    pub is_fluid: bool,
    pub icon: String,
}

impl Item {
    pub fn new(base: SerializableItem) -> Item {
        let mut hasher = DefaultHasher::new();
        base.id.hash(&mut hasher);
        let hash_id = hasher.finish();
        let mut hasher = DefaultHasher::new();
        base.prototype.hash(&mut hasher);
        let hash_proto = hasher.finish();
        let icon = format!("{}.png", base.id.clone());
        Item {
            uuid: hash_id,
            id: base.id,
            name: base.name,
            prototype: hash_proto,
            stack_size: base.stack_size,
            is_fluid: base.is_fluid,
            icon: icon,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{:#018x}: {}", self.uuid, self.id))?;
        //f.write_str(self.name.as_str())?;
        return Result::Ok(());
    }
}
