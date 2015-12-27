mod lib;
mod player;

extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use player::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();

    let name = "HalloWeltbeffepdejaijsnd".to_string();
    let mut level = lib::MyLevel::new(&renderer);

    let mut player_texture = renderer.create_texture(sdl2::pixels::PixelFormatEnum::RGB888,
                                                     sdl2::render::TextureAccess::Target,
                                                     (16, 32)).unwrap();
    renderer.render_target().unwrap().set(player_texture);
    renderer.clear();
    level.tileset.draw_tile(&mut renderer, 1345, 0, 0, 1);
    level.tileset.draw_tile(&mut renderer, 1409, 0, 16, 1);
    let player_texture = renderer.render_target().unwrap().reset();

    let mut player = Player{x:10.0, y:10.0, texture:player_texture.unwrap().unwrap()};

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        for r in 0..level.numRows {
            for c in 0..level.numColumns {
                level.tileset.draw_tile(&mut renderer,
                                level.tiles[(r * level.numColumns + c) as usize],
                                (c * level.tileset.tile_width * 3) as i32,
                                (r * level.tileset.tile_height * 3) as i32,
                                3);
            }
        }

        player.update();
        player.draw(&mut renderer);

        renderer.present();
    }
}
