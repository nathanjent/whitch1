use crate::{actor::Actor, util};
use agb::{
    fixnum::{num, Rect},
    input::{Button, ButtonController},
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

                if input.is_pressed(Button::B) {
                    // jump
                    actor.velocity.y += actor.acceleration.y;
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
                    actor.velocity.y = 0.into();
                }
            }
        }
    }
}
