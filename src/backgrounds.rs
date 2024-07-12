use crate::Vector2D;
use agb::{
    display::tiled::{RegularBackgroundSize, RegularMap, TiledMap, VRamManager},
    include_background_gfx,
};

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

/// The 4 quadrants to load map tile data into vram.
///
/// | 0 | 1 |
/// | 2 | 3 |
pub enum ScreenBlock {
    B0,
    B1,
    B2,
    B3,
}

pub fn load_level_background(
    map: &mut RegularMap,
    vram_manager: &mut VRamManager,
    level_number: usize,
    tilemap_offset: Vector2D<u16>,
    screen_block: ScreenBlock,
) {
    let level_map = tilemaps::LEVELS_MAP[level_number];
    let level_tileset = &backgrounds::level.tiles;

    // Tiles can only be loaded into one of 4 screen blocks
    // The screen block may not display correctly if the background size isn't set to match
    let (x1, x2, y1, y2) = match screen_block {
        ScreenBlock::B0 => (0u16, 32, 0, 32),
        ScreenBlock::B1 => (32, 64, 0, 32),
        ScreenBlock::B2 => (0, 32, 32, 64),
        ScreenBlock::B3 => (32, 64, 32, 64),
    };

    for y in y1..y2 {
        for x in x1..x2 {
            let (tile_x, tile_y) = match screen_block {
                ScreenBlock::B0 => (x + tilemap_offset.x, y + tilemap_offset.y),
                ScreenBlock::B1 => (x + tilemap_offset.x + 32, y + tilemap_offset.y),
                ScreenBlock::B2 => (x + tilemap_offset.x, y + tilemap_offset.y + 32),
                ScreenBlock::B3 => (x + tilemap_offset.x + 32, y + tilemap_offset.y + 32),
            };
            let tile_pos = (tile_y * 32 + tile_x) as usize;
            if tile_pos >= level_map.len() {
                return;
            }

            let tile_setting = level_map[tile_pos];

            map.set_tile(vram_manager, (x, y), &level_tileset, tile_setting);
        }
    }
}
