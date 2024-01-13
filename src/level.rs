use crate::{resources, behaviors::Behavior};
use agb::{display::object::Tag, fixnum::{Vector2D, Rect}};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum EntityType {
    Player,
    Bat,
    Door,
}

impl EntityType {
    pub fn tag(&self) -> &'static Tag {
        match self {
            EntityType::Player => resources::W_IDLE,
            EntityType::Bat => resources::BAT,
            EntityType::Door => resources::DOOR,
        }
    }
}

pub struct Entity(pub EntityType, pub Vector2D<i32>, pub Option<Vector2D<i32>>, pub &'static [Behavior]);

pub struct Level {
    pub width: u32,
    pub height: u32,
    pub starting_positions: &'static [Entity],
    pub name: &'static str,
    pub collision_rects: &'static [Rect<i32>],
}

impl Level {
    #[allow(unused_variables)]
    const fn new(
        width: u32,
        height: u32,
        starting_positions: &'static [Entity],
        name: &'static str,
        collision_rects: &'static [Rect<i32>],
    ) -> Self {
        Self {
            width,
            height,
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
    use crate::level::Level;
    use crate::behaviors::Behavior;
    use crate::level::{Entity, EntityType};

    include!(concat!(env!("OUT_DIR"), "/levels.rs"));
}
