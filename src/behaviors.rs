use agb::{
    fixnum::{FixedNum, Vector2D},
    input::ButtonController,
};

pub enum Behavior {
    Input,
}

impl Behavior {
    pub fn update(
        &self,
        position: &mut Vector2D<FixedNum<8>>,
        velocity: &mut Vector2D<FixedNum<8>>,
        input: &ButtonController,
    ) {
        match self {
            Self::Input => {
                match input.x_tri() {
                    agb::input::Tri::Positive => position.x += 1,
                    agb::input::Tri::Negative => position.x -= 1,
                    agb::input::Tri::Zero => (),
                }

                match input.y_tri() {
                    agb::input::Tri::Positive => position.y += 1,
                    agb::input::Tri::Negative => position.y -= 1,
                    agb::input::Tri::Zero => (),
                }
            }
        }
    }
}
