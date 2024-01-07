use crate::{actor::Actor, util};
use agb::{
    fixnum::{num, Rect},
    input::{Button, ButtonController}, mgba::Mgba,
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

                if actor.velocity.y == 0.into() && input.is_pressed(Button::B) {
                    if let Some(mut l) = logger {
                        let _ = l.print(format_args!("jump"), agb::mgba::DebugLevel::Debug);
                    }
                    // jump
                    actor.velocity.y -= actor.acceleration.y;
                }
            }
            Self::Gravity => {
                if actor.velocity.y < actor.max_velocity.y {
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
                        1.into(),
                    )
                }) {
                    actor.collision_mask.position.y -= actor.velocity.y;
                    actor.velocity.y = 0.into();
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
                    actor.collision_mask.position.x -= actor.velocity.x;
                    actor.velocity.x = 0.into();
                }
            }
        }
    }
}
