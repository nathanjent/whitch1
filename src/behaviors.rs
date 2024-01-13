use crate::{
    actor::{Actor, ActorState},
    sfx::Sfx,
    util,
};
use agb::{
    fixnum::{num, Rect},
    input::{Button, ButtonController},
    mgba::Mgba,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Behavior {
    Input,
    Gravity,
    Collider,
}

impl Behavior {
    pub fn update(
        &self,
        actor: &mut Actor,
        input: &ButtonController,
        collision_rects: &[Rect<i32>],
        sfx: &mut Sfx,
    ) {
        let logger = Mgba::new();
        match self {
            Self::Input => {
                match input.x_tri() {
                    agb::input::Tri::Positive => {
                        if actor.velocity.x < actor.max_velocity.x {
                            actor.velocity.x += actor.acceleration.x;
                        }
                    }
                    agb::input::Tri::Negative => {
                        if actor.velocity.x > -actor.max_velocity.x {
                            actor.velocity.x -= actor.acceleration.x;
                        }
                    }
                    agb::input::Tri::Zero => {
                        actor.velocity.x = util::lerp(0.into(), actor.velocity.x, num!(0.05))
                    }
                }

                if actor.state != ActorState::Jumping
                    && actor.velocity.y == 0.into()
                    && input.is_just_pressed(Button::B)
                {
                    // jump
                    // v_0 = (2 * h * v_x) / x_h
                    // g = (-2 * h * v_x^2) / x_h^2
                    //
                    // pos += vel * dt + 0.5 * acc * dt * dt
                    // vel += acc * dt
                    actor.state = ActorState::Jumping;
                    actor.velocity.y -= actor.max_velocity.y;
                    actor.jump_height += 1;
                    actor.jump_time += 1;
                    sfx.jump();
                }

                if actor.velocity == (0, 0).into() {
                    actor.state = ActorState::Idle;
                }
            }
            Self::Gravity => {
                if actor.velocity.y <= actor.max_velocity.y {
                    actor.velocity.y += actor.acceleration.y;
                }
            }
            Self::Collider => {
                if collision_rects.iter().any(|Rect { position, size }| {
                    let position = *position;
                    let size = *size;
                    actor.hit_bottom(
                        Rect {
                            position: position.into(),
                            size: size.into(),
                        },
                        num!(0.5),
                    )
                }) {
                    actor.velocity.y = 0.into();
                    actor.state = ActorState::Idle;
                    actor.jump_height = 0.into();
                    actor.jump_time = 0.into();
                }

                if collision_rects.iter().any(|Rect { position, size }| {
                    let position = *position;
                    let size = *size;
                    actor.hit_wall(
                        Rect {
                            position: position.into(),
                            size: size.into(),
                        },
                        3.into(),
                    )
                }) {
                    actor.velocity.x = 0.into();
                }
            }
        }

        if let Some(mut l) = logger {
            let _ = l.print(
                format_args!("actor_state: {:?}", actor.state),
                agb::mgba::DebugLevel::Debug,
            );
        }
    }
}
