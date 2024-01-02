use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::display::object::Tag;
use agb::fixnum::{num, FixedNum, Rect, Vector2D};

pub struct Actor<'a> {
    pub tag: &'a Tag,
    pub position: Vector2D<FixedNum<8>>,
    pub velocity: Vector2D<FixedNum<8>>,
    pub acceleration: Vector2D<FixedNum<8>>,
    pub max_velocity: Vector2D<FixedNum<8>>,
    pub collision_mask: Option<Rect<FixedNum<8>>>,
    pub visible: bool,
    frame: usize,
}

impl<'a> Actor<'a> {
    pub fn new(
        tag: &'a Tag,
        collision_mask: Option<Rect<FixedNum<8>>>,
        position: Vector2D<FixedNum<8>>,
        acceleration: Option<Vector2D<FixedNum<8>>>,
        max_velocity: Option<Vector2D<FixedNum<8>>>,
    ) -> Self {
        Self {
            tag,
            position,
            velocity: (0, 0).into(),
            acceleration: acceleration.unwrap_or((num!(0.2), num!(0.2)).into()),
            max_velocity: max_velocity.unwrap_or((1, 1).into()),
            collision_mask,
            visible: true,
            frame: 0,
        }
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
