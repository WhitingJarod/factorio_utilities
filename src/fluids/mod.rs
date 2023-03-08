use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::utils;

pub mod fluid;
use fluid::Fluid;

lazy_static! {
    pub static ref FLUIDS: HashMap<u64, Fluid> = {
        utils::read_to_hashmap("./resources/definitions/fluids/")
    };
}
