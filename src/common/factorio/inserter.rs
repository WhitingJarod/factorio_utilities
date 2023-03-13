use atomize::Atom;

use super::energy_source::EnergySource;

pub struct Inserter {
    id: Atom,
    energy_per_movement: u32,
    energy_source: EnergySource,
    extension_speed: f32,
    rotation_speed: f32,
    stack: bool,
}
