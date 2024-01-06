use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::display::object::Tag;
use agb::fixnum::{num, FixedNum, Rect, Vector2D};

type Fixed8 = FixedNum<8>;

pub struct Actor<'a> {
    pub tag: &'a Tag,
    pub velocity: Vector2D<Fixed8>,
    pub acceleration: Vector2D<Fixed8>,
    pub max_velocity: Vector2D<Fixed8>,
    pub collision_mask: Rect<Fixed8>,
    pub visible: bool,
    frame: usize,
}

impl<'a> Actor<'a> {
    pub fn new(
        tag: &'a Tag,
        collision_mask: Option<Rect<Fixed8>>,
        acceleration: Option<Vector2D<Fixed8>>,
        max_velocity: Option<Vector2D<Fixed8>>,
    ) -> Self {
        Self {
            tag,
            velocity: (0, 0).into(),
            acceleration: acceleration.unwrap_or((num!(0.2), num!(0.2)).into()),
            max_velocity: max_velocity.unwrap_or((1, 1).into()),
            collision_mask: collision_mask.unwrap_or(Rect {
                position: (0, 0).into(),
                size: (1, 1).into(),
            }),
            visible: true,
            frame: 0,
        }
    }

    pub fn render(&mut self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        let sprite = loader.get_vram_sprite(self.tag.animation_sprite(self.frame / 16));
        let mut obj = ObjectUnmanaged::new(sprite);
        obj.show().set_position(Vector2D {
            x: self.collision_mask.position.x.trunc(),
            y: self.collision_mask.position.y.trunc(),
        });
        if let Some(slot) = oam.next() {
            slot.set(&obj);
        }
    }

    pub fn aabb(&self) -> (Fixed8, Fixed8, Fixed8, Fixed8) {
        let Vector2D { x, y } = self.collision_mask.position;
        let Vector2D {
            x: width,
            y: height,
        } = self.collision_mask.size;
        (x, y, x + width, y + height)
    }

    pub fn hit_bottom(&self, collision_rect: Rect<FixedNum<8>>, sampling: Fixed8) -> bool {
        let (min_x, _, max_x, max_y) = self.aabb();
        let mut x = min_x;
        while x <= max_x {
            if collision_rect.contains_point((x.into(), max_y).into()) {
                return true;
            }
            x += sampling;
        }
        false
    }

    pub fn touches(&self, collision_rect: Rect<FixedNum<8>>) -> bool {
        collision_rect.touches(self.collision_mask)
    }
}
