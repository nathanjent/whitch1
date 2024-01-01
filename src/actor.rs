use crate::behaviors::Behavior;
use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::display::object::Tag;
use agb::fixnum::{FixedNum, Rect, Vector2D};
use agb::input::ButtonController;
use generational_arena::Arena;

pub struct Actor<'a> {
    pub tag: &'a Tag,
    pub position: Vector2D<FixedNum<8>>,
    pub velocity: Vector2D<FixedNum<8>>,
    pub collision_mask: Option<Rect<FixedNum<8>>>,
    pub visible: bool,
    pub behaviors: Arena<Behavior>,
    frame: usize,
}

impl<'a> Actor<'a> {
    pub fn new(
        tag: &'a Tag,
        collision_mask: Option<Rect<FixedNum<8>>>,
        position: Vector2D<FixedNum<8>>,
    ) -> Self {
        Self {
            tag,
            position,
            velocity: (0, 0).into(),
            collision_mask,
            visible: true,
            behaviors: Arena::with_capacity(100),
            frame: 0,
        }
    }

    pub fn update(&mut self, input: &mut ButtonController) {
        // Pass individual properties to the behavior to avoid extra borrows
        let mut position = self.position;
        let mut velocity = self.velocity;
        for (_, behavior) in self.behaviors.iter_mut() {
            behavior.update(&mut position, &mut velocity, input);
        }

        self.position = position;
        self.velocity = velocity;
    }

    pub fn render(&mut self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        let sprite = loader.get_vram_sprite(self.tag.animation_sprite(self.frame / 16));
        let mut obj = ObjectUnmanaged::new(sprite);
        obj.show().set_position(Vector2D {
            x: self.position.x.trunc(),
            y: self.position.y.trunc(),
        });
        if let Some(slot) = oam.next() {
            slot.set(&obj);
        }
    }
}
