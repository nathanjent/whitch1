#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::HEIGHT;
use agb::display::WIDTH;
use agb::display::object::ObjectUnmanaged;
use agb::display::object::TextAlignment;
use agb::display::tiled::TiledMap;
use agb::fixnum::FixedNum;
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

use agb::display::object::ObjectTextRender;
use agb::display::object::PaletteVram;
use agb::display::object::Size;
use agb::display::palette16::Palette16;
use agb::display::tiled::RegularBackgroundSize;
use agb::display::tiled::TileFormat;
use agb::display::Font;
use agb::display::Priority;
use agb::include_font;
use agb::interrupt::VBlank;
use agb::sound::mixer::Frequency;
use core::include_bytes;
use game::Game;
use level::Level;

const FONT: Font = include_font!("fonts/yoster.ttf", 12);

pub fn entry(mut gba: agb::Gba) -> ! {
    let vblank = VBlank::get();
    let (mut unmanaged, mut sprite_loader) = gba.display.object.get_unmanaged();
    //let managed = gba.display.object.get_managed();

    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    mixer.enable();

    let mut sfx = sfx::Sfx::new(&mut mixer);

    let (tiled, mut vram) = gba.display.video.tiled0();

    let mut input = agb::input::ButtonController::new();

    backgrounds::load_palettes(&mut vram);

    let current_level = 0;
    loop {
        let mut level_bg = tiled.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        backgrounds::load_level_background(&mut level_bg, &mut vram, current_level);
        let level = Level::get_level(current_level);

        let mut game = Game::new(level);

        //let mut bat_sprite = managed.object_sprite(resources::BAT.animation_sprite(0));
        let bat_sprite = sprite_loader.get_vram_sprite(resources::BAT.sprite(0));
        let mut bat_object = ObjectUnmanaged::new(bat_sprite);

        let mut position: Vector2D<i32> = (30,30).into();
        loop {
            sfx.frame();
            vblank.wait_for_vblank();
            input.update();

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

            // unmanaged
            let oam = &mut unmanaged.iter();
            bat_object.show().set_position(position);
            if let Some(slot) = oam.next() {
                slot.set(&bat_object);
            }

            game.update();

            level_bg.commit(&mut vram);
            level_bg.show();

            // managed
            //bat_sprite.set_position((40,40).into());
            //bat_sprite.show();

            //managed.commit();
        }
        //game.clear(&mut vram);
    }
}
