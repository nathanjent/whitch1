use crate::{actor::ActorState, behaviors::Behavior, resources};
use agb::{
    display::object::Tag,
    fixnum::{Rect, Vector2D},
    hash_map::HashMap,
};
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum EntityType {
    Player,
    Bat,
}

impl EntityType {
    pub fn tags(&self) -> HashMap<ActorState, &'static Tag> {
        let mut tags = HashMap::new();
        match self {
            EntityType::Player => {
                tags.insert(ActorState::Idle, resources::W_IDLE);
                tags.insert(ActorState::Running, resources::W_RUN);
                tags.insert(ActorState::Jumping, resources::W_JUMP);
            }
            EntityType::Bat => {
                tags.insert(ActorState::Idle, resources::BAT);
            }
        }

        tags
    }
}

pub struct Entity(
    pub EntityType,
    pub Vector2D<i32>,
    pub Option<Vector2D<i32>>,
    pub &'static [Behavior],
    pub Vector2D<i32>,
);

pub struct Level {
    pub width: u32,
    pub height: u32,
    pub starting_positions: &'static [Entity],
    pub name: &'static str,
    pub collision_rects: &'static [CollisionRect],
}

pub enum CollisionRect {
    Solid(Rect<i32>),
    Door(Rect<i32>),
}

impl Level {
    #[allow(unused_variables)]
    const fn new(
        width: u32,
        height: u32,
        starting_positions: &'static [Entity],
        name: &'static str,
        collision_rects: &'static [CollisionRect],
    ) -> Self {
        Self {
            width,
            height,
            starting_positions,
            name,
            collision_rects,
        }
    }

    pub fn get_solid_collision_rects(&self) -> Vec<Rect<i32>> {
        self.collision_rects
            .iter()
            .filter_map(|r| match r {
                CollisionRect::Solid(rect) => Some(*rect),
                _ => None,
            })
            .collect()
    }

    pub fn get_door_collision_rects(&self) -> Vec<Rect<i32>> {
        self.collision_rects
            .iter()
            .filter_map(|r| match r {
                CollisionRect::Door(rect) => Some(*rect),
                _ => None,
            })
            .collect()
    }

    pub fn get_level(level_number: usize) -> &'static Level {
        &levels::LEVELS[level_number]
    }
}

mod levels {
    use crate::behaviors::Behavior;
    use crate::level::{CollisionRect, Level};
    use crate::level::{Entity, EntityType};
    use agb::fixnum::{Rect, Vector2D};

    include!(concat!(env!("OUT_DIR"), "/levels.rs"));
}
