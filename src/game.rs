use alloc::vec;
use crate::behaviors::Behavior;
use crate::level::EntityType;
use crate::sfx::Sfx;
use agb::display::object::OamIterator;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::SpriteLoader;
use agb::fixnum::num;
use agb::input::ButtonController;
use alloc::vec::Vec;
use slotmap::Key;
use slotmap::new_key_type;
use slotmap::SecondaryMap;
use slotmap::SlotMap;

use crate::actor::Actor;
use crate::level::Entity;
use crate::level::Level;

new_key_type! { pub struct ActorKey; }

pub struct Game<'a> {
    level: &'a Level,
    input: ButtonController,
    actors: SlotMap<ActorKey, Actor<'a>>,
    behaviors: SecondaryMap<ActorKey, &'a [Behavior]>,
    player: ActorKey,
    enemies: Vec<ActorKey>,
    frame: usize,
    render_cache: Vec<RenderCache>,
}

impl<'a> Game<'a> {
    pub fn new(level: &'a Level) -> Self {
        Self {
            level,
            input: ButtonController::new(),
            actors: SlotMap::with_capacity_and_key(100),
            behaviors: SecondaryMap::with_capacity(100),
            player: ActorKey::null(),
            enemies: vec![ActorKey::null(); 100],
            frame: 0,
            render_cache: Vec::with_capacity(100),
        }
    }

    pub fn load_level(&mut self) {
        for Entity(entity, position, maybe_size, behaviors, sprite_offset) in
            self.level.starting_positions
        {
            let position = *position;
            let maybe_size = *maybe_size;
            let offset = *sprite_offset;
            let key = match entity {
                EntityType::Player => {
                    let actor = Actor::new(
                        entity.tag(),
                        position.into(),
                        maybe_size.map(|size| size.into()),
                        offset.into(),
                        Some((num!(1.4), num!(7.0)).into()),
                        Some((num!(1.0), num!(0.6)).into()),
                    );
                    let key = self.actors.insert(actor);
                    self.player = key;
                    key
                }
                EntityType::Bat => {
                    let actor = Actor::new(
                        entity.tag(),
                        position.into(),
                        maybe_size.map(|size| size.into()),
                        offset.into(),
                        Some((num!(1.4), num!(0.06)).into()),
                        Some((num!(1.0), num!(0.008)).into()),
                    );
                    let key = self.actors.insert(actor);
                    self.enemies.push(key);
                    key
                }
                EntityType::Door => {
                    let actor = Actor::new(
                        entity.tag(),
                        position.into(),
                        maybe_size.map(|size| size.into()),
                        offset.into(),
                        None,
                        None,
                    );
                    let key = self.actors.insert(actor);
                    key
                }
            };

            self.behaviors.insert(key, *behaviors);
        }
    }

    pub fn update(&mut self, sprite_loader: &mut SpriteLoader, sfx: &mut Sfx) {
        self.input.update();
        self.frame = self.frame.wrapping_add(1);

        let keys: Vec<ActorKey> = self.actors.keys().collect();
        for key in keys {
            if let Some(behaviors_for_actor) = self.behaviors.get(key) {
                for behavior in behaviors_for_actor.iter() {
                    behavior.update(
                        key,
                        self.player,
                        &*self.enemies,
                        &mut self.actors,
                        &self.input,
                        self.level.collision_rects,
                        sfx,
                    );
                }
            }
            if let Some(actor) = self.actors.get_mut(key) {
                actor.collision_mask.position += actor.velocity;
            }
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

        for (_, actor) in self.actors.iter() {
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
