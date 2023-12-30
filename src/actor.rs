use crate::behaviors::Behavior;
use agb::display::object::OamIterator;
use agb::display::object::SpriteLoader;
use agb::display::object::Tag;
use agb::fixnum::{FixedNum, Rect, Vector2D};
use generational_arena::Arena;

pub struct Actor<'a> {
    pub tag: &'a Tag,
    pub position: Vector2D<FixedNum<8>>,
    pub velocity: Vector2D<FixedNum<8>>,
    pub collision_mask: Rect<FixedNum<8>>,
    pub visible: bool,
    pub behaviors: Arena<Behavior>,
}

impl<'a> Actor<'a> {
    pub fn new(tag: &'a Tag, collision_mask: Rect<FixedNum<8>>) -> Self {
        Self {
            tag,
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
