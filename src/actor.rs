use agb::display::object::SpriteLoader;
use agb::display::object::OamIterator;
use crate::behaviors::Behavior;
use agb::{
    display::object::Object,
    fixnum::{FixedNum, Rect, Vector2D},
};
use generational_arena::Arena;

pub struct Actor<'a> {
    sprite: Object<'a>,
    position: Vector2D<FixedNum<8>>,
    velocity: Vector2D<FixedNum<8>>,
    collision_mask: Rect<FixedNum<8>>,
    visible: bool,
    behaviors: Arena<Behavior>,
}

impl<'a> Actor<'a> {
    pub fn new(sprite: Object<'a>, collision_mask: Rect<FixedNum<8>>) -> Self {
        Self {
            sprite,
            position: (0, 0).into(),
            velocity: (0, 0).into(),
            collision_mask,
            visible: true,
            behaviors: Arena::with_capacity(100),
        }
    }

    pub fn update(&mut self) {
        for (i, behavior) in self.behaviors.iter_mut() {
            behavior.update();
        }
    }

    pub fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
    }
}
