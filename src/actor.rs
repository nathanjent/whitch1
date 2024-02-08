use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::display::object::Tag;
use agb::fixnum::{num, FixedNum, Rect, Vector2D};
use agb::input::Tri;

type Number = FixedNum<8>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ActorState {
    Idle,
    Jumping,
    Falling,
    Running,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Action {
    None,
    Jump,
    Attack,
}

pub struct Actor<'a> {
    pub tag: &'a Tag,
    pub velocity: Vector2D<Number>,
    pub acceleration: Vector2D<Number>,
    pub max_velocity: Vector2D<Number>,
    pub collision_mask: Rect<Number>,
    pub sprite_offset: Vector2D<Number>,
    pub visible: bool,
    pub state: ActorState,
    pub current_action: Action,
    pub direction_x: Tri,
    pub facing: Tri,
    pub jump_height: Number,
    pub jump_time: Number,
    pub jump_distance_to_peak: Number,
    frame: usize,
    health: Number,
}

impl<'a> Actor<'a> {
    pub fn new(
        tag: &'a Tag,
        position: Vector2D<Number>,
        maybe_size: Option<Vector2D<Number>>,
        offset: Vector2D<Number>,
        max_velocity: Option<Vector2D<Number>>,
        acceleration: Option<Vector2D<Number>>,
    ) -> Self {
        Self {
            tag,
            sprite_offset: offset,
            velocity: (0, 0).into(),
            acceleration: acceleration.unwrap_or((num!(0.2), num!(0.2)).into()),
            max_velocity: max_velocity.unwrap_or((1, 1).into()),
            collision_mask: maybe_size.map_or(
                Rect {
                    position,
                    size: (1, 1).into(),
                },
                |size| Rect { position, size },
            ),
            visible: true,
            state: ActorState::Idle,
            current_action: Action::None,
            frame: 0,
            jump_height: 0.into(),
            jump_time: 0.into(),
            jump_distance_to_peak: 0.into(),
            direction_x: Tri::Zero,
            facing: Tri::Zero,
            health: num!(100.0),
        }
    }

    pub fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        let sprite = loader.get_vram_sprite(self.tag.animation_sprite(self.frame / 16));
        let mut obj = ObjectUnmanaged::new(sprite);
        let position = self.collision_mask.position + self.sprite_offset;
        obj.show()
            .set_position(Vector2D {
                x: position.x.trunc(),
                y: position.y.trunc(),
            })
            .set_hflip(self.facing == Tri::Negative);
        if let Some(slot) = oam.next() {
            slot.set(&obj);
        }
    }

    pub fn aabb(&self) -> (Number, Number, Number, Number) {
        let Vector2D { x, y } = self.collision_mask.position + self.velocity;
        let Vector2D {
            x: width,
            y: height,
        } = self.collision_mask.size;
        (x, y, x + width, y + height)
    }

    pub fn hit_ground(&self, collision_rects: &[Rect<i32>], sampling: Number) -> bool {
        collision_rects.iter().any(|Rect { position, size }| {
            let position = *position;
            let size = *size;
            let collision_rect: Rect<Number> = Rect {
                position: position.into(),
                size: size.into(),
            };

            let (min_x, _, max_x, max_y) = self.aabb();
            let mut x = min_x + sampling;
            while x <= max_x - sampling {
                if collision_rect.contains_point((x.into(), max_y).into()) {
                    return true;
                }
                x += sampling;
            }
            false
        })
    }

    pub fn hit_ceiling(&self, collision_rects: &[Rect<i32>], sampling: Number) -> bool {
        collision_rects.iter().any(|Rect { position, size }| {
            let position = *position;
            let size = *size;
            let collision_rect: Rect<Number> = Rect {
                position: position.into(),
                size: size.into(),
            };

            let (min_x, min_y, max_x, _) = self.aabb();
            let mut x = min_x + sampling;
            while x <= max_x - sampling {
                if collision_rect.contains_point((x.into(), min_y).into()) {
                    return true;
                }
                x += sampling;
            }
            false
        })
    }

    pub fn hit_wall(&self, collision_rects: &[Rect<i32>], sampling: Number) -> bool {
        collision_rects.iter().any(|Rect { position, size }| {
            let position = *position;
            let size = *size;
            let collision_rect: Rect<Number> = Rect {
                position: position.into(),
                size: size.into(),
            };

            let (min_x, min_y, max_x, max_y) = self.aabb();
            let mut y = min_y;
            while y < max_y - self.velocity.y {
                let x = if self.velocity.x.to_raw().is_negative() {
                    min_x
                } else {
                    max_x
                };
                if collision_rect.contains_point((x, y.into()).into()) {
                    return true;
                }
                y += sampling;
            }
            false
        })
    }

    pub fn take_damage(&mut self) {
        self.health -= 1;
    }
}
