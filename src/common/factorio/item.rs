use atomize::Atom;

pub struct Item {
    id: Atom,
    stack_size: u16,
}

impl Item {
    fn get_id(&self) -> Atom {
        self.id
    }
    fn get_stack_size(&self) -> u16 {
        self.stack_size
    }
}
