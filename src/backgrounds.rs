use agb::display::tiled::RegularMap;
use agb::{include_background_gfx, println};
use agb::display::tiled::VRamManager;

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

pub fn load_level_background(
    map: &mut RegularMap,
    vram_manager: &mut VRamManager,
    level_number: usize,
) {
    let level_map = &tilemaps::LEVELS_MAP[level_number];

    let level_tileset = backgrounds::level.tiles;

    for y in 0..32u16 {
        for x in 0..32u16 {
            let tile_pos = y * 32 + x;
            let tile_setting = level_map[tile_pos as usize];

            map.set_tile(vram_manager, (x, y).into(), &level_tileset, tile_setting);
        }
    }
}

