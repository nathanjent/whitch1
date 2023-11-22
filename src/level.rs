use agb::{display::object::Tag, fixnum::Vector2D};
use crate::resources;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Item {
    Player,
    Bat,
    Door,
}

impl Item {
    pub fn shadow_tag(&self) -> &'static Tag {
        match self {
            Item::Player => resources::PLAYER,
            Item::Bat => resources::BAT,
            Item::Door => resources::DOOR,
        }
    }

    pub fn tag(&self) -> &'static Tag {
        match self {
            Item::Player => resources::PLAYER,
            Item::Bat => resources::BAT,
            Item::Door => resources::DOOR,
        }
    }

    pub fn map_entity_offset(&self) -> Vector2D<i32> {
        const STANDARD: Vector2D<i32> = Vector2D::new(0, -3);
        const ZERO: Vector2D<i32> = Vector2D::new(0, 0);

        match self {
            Item::Player => STANDARD,
            Item::Bat => STANDARD,
            Item::Door => ZERO,
        }
    }
}

pub struct Entity(pub Item, pub Vector2D<i32>);

pub struct Level {
    pub starting_positions: &'static [Entity],
    pub name: &'static str,
}

impl Level {
    #[allow(unused_variables)]
    const fn new(
        starting_positions: &'static [Entity],
        name: &'static str,
    ) -> Self {
        Self {
            starting_positions,
            name,
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
    use super::*;
    use agb::fixnum::Vector2D;

    include!(concat!(env!("OUT_DIR"), "/levels.rs"));
}

