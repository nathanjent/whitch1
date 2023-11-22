#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

mod backgrounds;
mod behaviors;
mod entity;
mod game;
mod level;
mod resources;
mod sfx;

use agb::display::tiled::RegularBackgroundSize;
use agb::display::tiled::TileFormat;
use agb::display::Priority;
use agb::fixnum::Rect;
use agb::interrupt::VBlank;
use agb::sound::mixer::Frequency;
use entity::Entity;
use game::Game;

pub fn entry(mut gba: agb::Gba) -> ! {
    let vblank = VBlank::get();
    vblank.wait_for_vblank();

    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    mixer.enable();

    let mut sfx = sfx::Sfx::new(&mut mixer);

    let (tiled, mut vram) = gba.display.video.tiled0();

    let mut level_bg = tiled.background(
        Priority::P1,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    backgrounds::load_palettes(&mut vram);

    let object = gba.display.object.get_managed();

    //let (unmanaged, sprite_loader) = gba.display.object.get_unmanaged();

    let mut input = agb::input::ButtonController::new();
    input.update();

    let mut game = Game::new();

    let mut bat_sprite = object.object_sprite(resources::BAT.sprite(0));
    bat_sprite.set_priority(Priority::P1);
    game.entities.insert(Entity::new(
        bat_sprite,
        Rect::new((0, 0).into(), (10, 10).into()),
    ));

    loop {
        sfx.frame();
        vblank.wait_for_vblank();

        game.update(&object);

        object.commit();
    }
}
