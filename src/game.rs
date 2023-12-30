use crate::level::Entity;
use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::fixnum::Rect;
use agb::fixnum::Vector2D;
use agb::input::ButtonController;
use alloc::vec::Vec;
use generational_arena::Arena;

use crate::actor::Actor;
use crate::level::EntityWithPosition;
use crate::level::Level;

pub struct Game<'a> {
    input: ButtonController,
    level: &'a Level,
    pub actors: Arena<Actor<'a>>,
    frame: usize,
    render_cache: Vec<RenderCache>,
}

impl<'a> Game<'a> {
    pub fn new(level: &'a Level) -> Self {
        Self {
            input: ButtonController::new(),
            level,
            actors: Arena::with_capacity(100),
            frame: 0,
            render_cache: Vec::with_capacity(100),
        }
    }

    //fn clear(&mut self, vram: &mut VRamManager) {
    //    self.level.clear(vram);
    //}

    pub fn load_level(&mut self) {
        for EntityWithPosition(entity, Vector2D { x, y }) in self.level.starting_positions {
            let actor = match entity {
                Entity::Player => {
                    let collision_mask = Rect::new((*x, *y).into(), (16, 16).into());
                    Actor::new(entity.tag(), collision_mask)
                }
                Entity::Bat => {
                    let collision_mask = Rect::new((*x, *y).into(), (16, 16).into());
                    Actor::new(entity.tag(), collision_mask)
                }
                Entity::Door => {
                    let collision_mask = Rect::new((*x, *y).into(), (16, 16).into());
                    Actor::new(entity.tag(), collision_mask)
                }
            };

            self.actors.insert(actor);
        }
    }

    pub fn update(&mut self, sprite_loader: &mut SpriteLoader) {
        self.frame = self.frame.wrapping_add(1);
        for (_, actor) in self.actors.iter_mut() {
            actor.update();
        }
        self.input.update();

        self.cache_render(sprite_loader);
    }

    fn cache_render(&mut self, sprite_loader: &mut SpriteLoader) {
        self.render_cache = self
            .actors
            .iter()
            .map(|(_, actor)| {
                let object = ObjectUnmanaged::new(
                    sprite_loader.get_vram_sprite(actor.tag.animation_sprite(self.frame / 16)),
                );
                RenderCache { object }
            })
            .collect();
        self.render_cache
            .sort_unstable_by_key(|r| r.sorting_number());
    }

    pub fn render(&self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        for item in self.render_cache.iter() {
            item.render(oam);
        }
    }
}

struct RenderCache {
    object: ObjectUnmanaged,
}

impl RenderCache {
    pub fn render(&self, oam: &mut OamIterator) {
        if let Some(slot) = oam.next() {
            slot.set(&self.object);
        }
    }

    pub fn sorting_number(&self) -> i32 {
        // TODO return z index based on actor type
        42
    }
}
