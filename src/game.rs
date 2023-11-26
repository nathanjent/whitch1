use agb::display::object::SpriteLoader;
use agb::display::object::OamIterator;
use agb::input::ButtonController;
use generational_arena::Arena;

use crate::actor::Actor;
use crate::level::Level;

pub struct Game<'a> {
    input: ButtonController,
    level: &'a Level,
    pub actors: Arena<Actor<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(level: &'a Level) -> Self {
        Self {
            input: ButtonController::new(),
            level,
            actors: Arena::with_capacity(100),
        }
    }

    //fn clear(&mut self, vram: &mut VRamManager) {
    //    self.level.clear(vram);
    //}
}

impl<'a> Game<'a> {
    pub fn update(&mut self) {
        for (_, actor) in self.actors.iter_mut() {
            actor.update();
        }
        self.input.update();
    }

    fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        for (_, actor) in self.actors.iter() {
            actor.render(loader, oam);
        }
    }
}
