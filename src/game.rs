use crate::behaviors::Behavior;
use crate::level::EntityType;
use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::fixnum::{num, Rect};
use agb::input::ButtonController;
use alloc::vec::Vec;
use generational_arena::Arena;

use crate::actor::Actor;
use crate::level::Entity;
use crate::level::Level;

pub struct Game<'a> {
    level: &'a Level,
    input: ButtonController,
    actors: Arena<Actor<'a>>,
    behaviors: Arena<Arena<Behavior>>,
    frame: usize,
    render_cache: Vec<RenderCache>,
}

impl<'a> Game<'a> {
    pub fn new(level: &'a Level) -> Self {
        Self {
            level,
            input: ButtonController::new(),
            actors: Arena::with_capacity(100),
            behaviors: Arena::with_capacity(100),
            frame: 0,
            render_cache: Vec::with_capacity(100),
        }
    }

    //fn clear(&mut self, vram: &mut VRamManager) {
    //    self.level.clear(vram);
    //}

    pub fn load_level(&mut self) {
        for Entity(entity, position, maybe_size, behaviors) in self.level.starting_positions {
            let position = *position;
            let collision_mask = maybe_size.map(|size| Rect::new(position.into(), size.into()));
            let actor = match entity {
                EntityType::Player | EntityType::Bat => Actor::new(
                    entity.tag(),
                    collision_mask,
                    position.into(),
                    Some((num!(1.0), num!(3.0)).into()),
                    Some((num!(0.2), num!(0.8)).into()),
                ),
                EntityType::Door => {
                    Actor::new(entity.tag(), collision_mask, position.into(), None, None)
                }
            };

            self.actors.insert(actor);
            self.behaviors
                .insert(behaviors.iter().map(|b| *b).collect());
        }
    }

    pub fn update(&mut self, sprite_loader: &mut SpriteLoader) {
        self.input.update();
        self.frame = self.frame.wrapping_add(1);

        for (idx, actor) in self.actors.iter_mut() {
            if let Some(behaviors) = self.behaviors.get(idx) {
                for (_, behavior) in behaviors.iter() {
                    behavior.update(actor, &self.input, self.level.collision_rects);
                }
            }
            actor.position += actor.velocity;
        }

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

    pub fn render(&mut self, loader: &mut SpriteLoader, oam: &mut OamIterator) {
        for item in self.render_cache.iter() {
            item.render(oam);
        }

        for (_, actor) in self.actors.iter_mut() {
            actor.render(loader, oam);
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
