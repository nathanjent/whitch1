use crate::resources;
use agb::{display::object::Tag, fixnum::{Vector2D, Rect}};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Entity {
    Player,
    Bat,
    Door,
}

impl Entity {
    pub fn shadow_tag(&self) -> &'static Tag {
        match self {
            Entity::Player => resources::W_IDLE,
            Entity::Bat => resources::BAT,
            Entity::Door => resources::DOOR,
        }
    }

    pub fn tag(&self) -> &'static Tag {
        match self {
            Entity::Player => resources::W_IDLE,
            Entity::Bat => resources::BAT,
            Entity::Door => resources::DOOR,
        }
    }

    pub fn map_entity_offset(&self) -> Vector2D<i32> {
        const STANDARD: Vector2D<i32> = Vector2D::new(0, -3);
        const ZERO: Vector2D<i32> = Vector2D::new(0, 0);

        match self {
            Entity::Player => STANDARD,
            Entity::Bat => STANDARD,
            Entity::Door => ZERO,
        }
    }
}

pub struct EntityWithPosition(pub Entity, pub Vector2D<i32>);

pub struct Level {
    pub starting_positions: &'static [EntityWithPosition],
    pub name: &'static str,
    pub collision_rects: &'static [Rect<i32>],
}

impl Level {
    #[allow(unused_variables)]
    const fn new(
        starting_positions: &'static [EntityWithPosition],
        name: &'static str,
        collision_rects: &'static [Rect<i32>],
    ) -> Self {
        Self {
            starting_positions,
            name,
            collision_rects,
        }
    }

    pub const fn get_level(level_number: usize) -> &'static Level {
        &levels::LEVELS[level_number]
    }

    pub const fn num_levels() -> usize {
        levels::LEVELS.len()
    }
}

mod levels {
    use agb::fixnum::{Vector2D, Rect};
    use crate::Level;
    use crate::level::{EntityWithPosition, Entity};

    include!(concat!(env!("OUT_DIR"), "/levels.rs"));
}
