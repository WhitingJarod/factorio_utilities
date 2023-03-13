use atomize::Atom;

use super::energy_source::EnergySource;

pub struct Crafter {
    id: Atom,
    crafting_categories: Vec<Atom>,
    crafting_speed: f32,
    energy_usage: u32,
    energy_source: EnergySource,
    module_slots: u16,
}
