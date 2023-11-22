use agb::include_background_gfx;
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

