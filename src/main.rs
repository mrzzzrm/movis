mod lib;

extern crate sdl2;
extern crate sdl2_image;


fn main() {
    let mut game = lib::Game::new();

    loop {
        if !game.update() {
            break;
        }
    }
}
