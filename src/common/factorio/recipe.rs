use atomize::Atom;

pub struct Recipe {
    id: Atom,
    category: Atom,
    energy_required: u16,
    ingredients: Vec<(Atom, u16)>,
    results: Vec<(Atom, u16)>,
}
