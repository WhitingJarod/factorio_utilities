use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::utils;

pub mod item;
use item::Item;

lazy_static! {
    pub static ref ITEMS: HashMap<u64, Item> = {
        utils::read_to_hashmap("./resources/definitions/items/")
    };
}
