#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![feature(slice_pattern)]

use core::fmt::Write;

extern crate alloc;

mod actor;
mod backgrounds;
mod behaviors;
mod close_to_zero;
mod game;
mod level;
mod resources;
mod sfx;
mod util;

use agb::display::object::ObjectTextRender;
use agb::display::object::PaletteVram;
use agb::display::object::Size;
use agb::display::object::TextAlignment;
use agb::display::palette16::Palette16;
use agb::display::WIDTH;
use agb::fixnum::Vector2D;
use agb::interrupt::VBlank;
use agb::mgba::Mgba;
use agb::sound::mixer::Frequency;

use game::Game;
use level::Level;

pub fn entry(mut gba: agb::Gba) -> ! {
    let mut logger = Mgba::new();
    let vblank = VBlank::get();
    let (mut unmanaged, mut sprite_loader) = gba.display.object.get_unmanaged();

    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    mixer.enable();

    let mut sfx = sfx::Sfx::new(&mut mixer);
    //sfx.crawl();

    let (tiled, mut vram) = gba.display.video.tiled0();

    backgrounds::load_palettes(&mut vram);

    let mut palette = [0x0; 16];
    palette[1] = 0xFF_FF;
    let palette = Palette16::new(palette);
    let palette = PaletteVram::new(&palette).unwrap();
    let mut writer = ObjectTextRender::new(&resources::FONT, Size::S16x16, palette);
    let _ = writeln!(writer, "Hello, World!");
    writer.layout((WIDTH, 40), TextAlignment::Left, 2);

    let current_level = 1;

    loop {
        let level = Level::get_level(current_level);
        let (mut bg2, mut bg3) = backgrounds::load_backgrounds(current_level, level, &tiled);

        let mut between_updates = || {
            sfx.frame();
            vblank.wait_for_vblank();
        };

        let start_pos = (0, 0).into();
        bg2.init(&mut vram, start_pos, &mut between_updates);
        bg3.init(&mut vram, start_pos, &mut between_updates);

        bg2.commit(&mut vram);
        bg3.commit(&mut vram);

        bg2.set_visible(true);
        bg3.set_visible(true);

        let mut game = Game::new(level);
        game.load_level_assets();

        loop {
            writer.next_letter_group();
            writer.update((0, 0));
            sfx.frame();

            vblank.wait_for_vblank();
            bg2.commit(&mut vram);
            bg3.commit(&mut vram);

            let oam = &mut unmanaged.iter();

            game.update(&mut sfx);

            // Update scroll
            bg2.set_pos(&mut vram, -game.scroll_pos);
            bg3.set_pos(
                &mut vram,
                -Vector2D {
                    x: (game.scroll_pos.x * 6) / 10,
                    y: (game.scroll_pos.y * 8) / 10,
                },
            );

            game.render(&mut sprite_loader, oam);

            writer.commit(oam);
        }
    }
}
