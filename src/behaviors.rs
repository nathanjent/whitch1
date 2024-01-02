use crate::{actor::Actor, util};
use agb::{ input::ButtonController, fixnum::{num, Rect}};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Behavior {
    Input,
    Gravity,
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
                    agb::input::Tri::Positive => if actor.velocity.x < actor.max_velocity.x {
                        actor.velocity.x += actor.acceleration.x;
                    },
                    agb::input::Tri::Negative => if actor.velocity.x > -actor.max_velocity.x {
                        actor.velocity.x -= actor.acceleration.x;
                    },
                    agb::input::Tri::Zero => actor.velocity.x = util::lerp(0.into(), actor.velocity.x, num!(0.05)),
                }

                actor.position += actor.velocity;
            }
            Self::Gravity => {}
        }
    }
}
