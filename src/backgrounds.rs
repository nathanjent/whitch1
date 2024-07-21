use crate::level::Level;
use agb::display::tiled::InfiniteScrolledMap;
use agb::display::tiled::TileFormat;
use agb::display::tiled::Tiled0;
use agb::display::Priority;
use agb::{
    display::tiled::{RegularBackgroundSize, VRamManager},
    include_background_gfx,
};
use alloc::boxed::Box;

include_background_gfx!(backgrounds, "1e151b",
    level => deduplicate "gfx/bg.png",
);

mod tilemaps {
    use super::backgrounds;
    include!(concat!(env!("OUT_DIR"), "/tilemaps.rs"));
}

pub fn load_palettes(vram_manager: &mut VRamManager) {
    vram_manager.set_background_palettes(backgrounds::PALETTES);
}

pub fn load_backgrounds<'a>(
    level_number: usize,
    level: &'a Level,
    tiled: &'a Tiled0,
) -> InfiniteScrolledMap<'a> {
    let level_map = tilemaps::LEVELS_MAP[level_number];
    let level_tileset = &backgrounds::level.tiles;

    let background = InfiniteScrolledMap::new(
        tiled.background(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        ),
        Box::new(|pos| {
            let index = (pos.x + level.width as i32 * pos.y) as usize;
            if index < level_map.len() {
                (level_tileset, level_map[index])
            } else {
                (level_tileset, level_map[0])
            }
        }),
    );

    background
}
