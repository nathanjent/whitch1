#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::WIDTH;
use agb::display::object::PaletteVram;
use agb::display::object::Size;
use agb::display::object::ObjectTextRender;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::TextAlignment;
use agb::display::palette16::Palette16;
use agb::display::tiled::TiledMap;
use agb::fixnum::Vector2D;

use core::fmt::Write;

extern crate alloc;

mod actor;
mod backgrounds;
mod behaviors;
mod game;
mod level;
mod resources;
mod sfx;

use agb::display::tiled::RegularBackgroundSize;
use agb::display::tiled::TileFormat;
use agb::display::Priority;
use agb::interrupt::VBlank;
use agb::sound::mixer::Frequency;
use game::Game;
use level::Level;


pub fn entry(mut gba: agb::Gba) -> ! {
    let vblank = VBlank::get();
    let (mut unmanaged, mut sprite_loader) = gba.display.object.get_unmanaged();

    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    mixer.enable();

    let mut sfx = sfx::Sfx::new(&mut mixer);

    let (tiled, mut vram) = gba.display.video.tiled0();

    backgrounds::load_palettes(&mut vram);

    let mut palette = [0x0; 16];
    palette[1] = 0xFF_FF;
    let palette = Palette16::new(palette);
    let palette = PaletteVram::new(&palette).unwrap();
    let mut writer = ObjectTextRender::new(&resources::FONT, Size::S16x16, palette);
    let _ = writeln!(writer, "Hello, World!");
    writer.layout((WIDTH, 40).into(), TextAlignment::Left, 2);

    let current_level = 0;
    loop {
        let mut level_bg = tiled.background(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        backgrounds::load_level_background(&mut level_bg, &mut vram, current_level);
        let level = Level::get_level(current_level);

        let mut game = Game::new(level);
        game.load_level();

        loop {
            writer.next_letter_group();
            writer.update((0, 0).into());
            sfx.frame();
            vblank.wait_for_vblank();


            let oam = &mut unmanaged.iter();

            game.update(&mut sprite_loader);

            level_bg.commit(&mut vram);
            level_bg.show();

            game.render(&mut sprite_loader, oam);

            writer.commit(oam);
        }
    }
}
