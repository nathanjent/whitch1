use agb::display::object::OamManaged;
use agb::input::ButtonController;
use generational_arena::Arena;

use crate::entity::Entity;

pub struct Game<'a> {
    input: ButtonController,
    pub entities: Arena<Entity<'a>>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        Self {
            input: ButtonController::new(),
            entities: Arena::with_capacity(100),
        }
    }
}

impl<'a> Game<'a> {
    pub fn update(&mut self, object: &'a OamManaged<'a>) {
        for (i, entity) in self.entities.iter_mut() {
            entity.update(object);
        }
    }
}
