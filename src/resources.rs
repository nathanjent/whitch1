use agb::{
    display::object::Graphics,
    include_aseprite,
};

const SPRITES: &Graphics = include_aseprite!(
    "gfx/whitch_design.aseprite",
    "gfx/enemies.aseprite",
    "gfx/objects.aseprite"
);

/// Define the tags from the aseprite files
macro_rules! named_tag {
    (
        $sprites:ident, [
            $($name:tt),+ $(,)?
        ] $(,)?
    ) => {
        $(
            pub const $name: &agb::display::object::Tag = $sprites.tags().get(stringify!($name));
        )+
    };
}

named_tag!(
    SPRITES,
    [
        W_IDLE,
        W_RUN,
        BAT,
        DOOR,
    ]
);

