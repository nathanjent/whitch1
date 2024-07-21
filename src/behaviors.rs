use agb::mgba::DebugLevel;
use crate::Vector2D;
use crate::{
    actor::{Action, Actor, ActorState},
    game::ActorKey,
    sfx::Sfx,
    util,
};
use agb::fixnum::{FixedNum, Num, Number};
use agb::{
    fixnum::{num, Rect},
    input::{Button, ButtonController, Tri},
    mgba::Mgba,
};
use slotmap::SlotMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Behavior {
    Input,
    Player,
    Flap,
}

fn vector_close_to_zero(v: &Vector2D<FixedNum<8>>, precision: FixedNum<8>) -> bool {
    (v.x < precision && v.x > -precision) && (v.y < precision && v.y > -precision)
}

impl Behavior {
    pub fn update(
        &self,
        current_key: ActorKey,
        player_key: ActorKey,
        enemies_keys: &[ActorKey],
        actors: &mut SlotMap<ActorKey, Actor>,
        input: &ButtonController,
        collision_rects: &[Rect<i32>],
        sfx: &mut Sfx,
    ) {
        let mut logger = Mgba::new();
        if input.is_just_pressed(Button::A) {
        }

        match self {
            Self::Input => {
                if let Some(actor) = actors.get_mut(current_key) {
                    actor.direction_x = input.x_tri();
                    if actor.state != ActorState::Jumping && input.is_just_pressed(Button::B) {
                        actor.current_action = Action::Jump;
                    }
                    if actor.state == ActorState::Jumping && input.is_just_released(Button::B) {
                        actor.current_action = Action::JumpCut;
                    }
                }
            }
            Self::Flap => {
                if let Some(actor) = actors.get_mut(current_key) {
                    if actor.current_action == Action::Jump {
                        actor.state = ActorState::Jumping;
                        actor.velocity.y -= actor.max_velocity.y;
                        sfx.bat_flap();
                    }

                    //actor.velocity.y += actor.acceleration.y;
                    if actor.collision_mask.position.y > num!(3.0) {
                        actor.current_action = Action::Jump;
                    }

                    actor.current_action = Action::None;
                }
            }
            Self::Player => {
                if let Some(actor) = actors.get_mut(current_key) {
                    let vx = actor.velocity.x;
                    match actor.direction_x {
                        Tri::Negative => {
                            if actor.velocity.x > -actor.max_velocity.x {
                                actor.velocity.x -= actor.acceleration.x;
                                if actor.state == ActorState::Idle {
                                    actor.state = ActorState::Running;
                                }
                                actor.facing = actor.direction_x;
                            }
                        }
                        Tri::Positive => {
                            if actor.velocity.x < actor.max_velocity.x {
                                actor.velocity.x += actor.acceleration.x;
                                if actor.state == ActorState::Idle {
                                    actor.state = ActorState::Running;
                                }
                                actor.facing = actor.direction_x;
                            }
                        }
                        Tri::Zero => {}
                    }
                    if vx == actor.velocity.x {
                        actor.velocity.x =
                            util::lerp(actor.velocity.x, 0.into(), actor.acceleration.x)
                    }

                    if actor.hit_wall(collision_rects, num!(3.0)) {
                        actor.velocity.x = 0.into();
                    }

                    if actor.hit_ground(collision_rects, num!(0.8)) {
                        actor.velocity.y = 0.into();
                        if actor.state == ActorState::Jumping {
                            actor.state = ActorState::Idle;
                        }
                    } else {
                        actor.velocity.y += actor.acceleration.y;
                    }

                    if actor.current_action == Action::Jump && actor.velocity.y == 0.into() {
                        // jump
                        // v_0 = (2 * h * v_x) / x_h
                        // g = (-2 * h * v_x^2) / x_h^2
                        //
                        // pos += vel * dt + 0.5 * acc * dt * dt
                        // vel += acc * dt
                        actor.state = ActorState::Jumping;
                        actor.velocity.y -= actor.max_velocity.y;
                        sfx.jump();
                    }

                    if actor.current_action == Action::JumpCut && actor.velocity.y != 0.into() {
                        actor.velocity.y = 0.into();
                    }

                    if actor.hit_ceiling(collision_rects, num!(0.8)) {
                        actor.velocity.y = 0.into();
                    }

                    if vector_close_to_zero(&actor.velocity, num!(0.02)) {
                        actor.velocity = (0, 0).into();
                        actor.state = ActorState::Idle;
                    }

                    actor.current_action = Action::None;
                }

                if enemies_keys.iter().any(|enemy_key: &ActorKey| {
                    let enemy = actors.get(*enemy_key);
                    let current = actors.get(current_key);

                    enemy
                        .map(|e| {
                            current
                                .map(|c| e.collision_mask.touches(c.collision_mask))
                                .is_some_and(|r| r)
                        })
                        .is_some_and(|r| r)
                }) {
                    if let Some(actor) = actors.get_mut(current_key) {
                        actor.take_damage();
                    }
                }
            }
        }

        //if let Some(actor) = actors.get(current_key) {
        //    logger.as_mut().and_then(|l| {
        //        l.print(
        //            format_args!(
        //                "actor_state: {:?} x: {} y: {} vx: {} vy: {}",
        //                actor.state,
        //                actor.collision_mask.position.x,
        //                actor.collision_mask.position.y,
        //                actor.velocity.x,
        //                actor.velocity.y,
        //            ),
        //            DebugLevel::Debug,
        //        )
        //        .ok()
        //    });
        //}
    }
}
