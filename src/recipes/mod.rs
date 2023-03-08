use lazy_static::lazy_static;
use std::collections::HashMap;
use colored::Colorize;

use crate::utils::{self, FactorioType};
use crate::items::ITEMS;

pub mod recipe;
use recipe::Recipe;

lazy_static! {
    pub static ref RECIPES: HashMap<u64, Recipe> = {
        let map: HashMap<u64, Recipe> = utils::read_to_hashmap("./resources/defintions/recipes/");

        for recipe in map.values() {
            for id in recipe.item_ingredients.keys() {
                if !ITEMS.contains_key(id) {
                    println!("{}{}", "Missing item definition required as ingredient for recipe ".truecolor(255, 195, 63), recipe.get_id().bright_yellow());
                }
            }
        }
        for recipe in map.values() {
            for id in recipe.item_products.keys() {
                if !ITEMS.contains_key(id) {
                    println!("{}{}", "Missing item definition required as product for recipe ".truecolor(255, 195, 63), recipe.get_id().bright_yellow());
                }
            }
        }
        //TODO: Definition checking for fluids.

        map
    };
}
