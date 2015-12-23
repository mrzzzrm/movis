extern crate ovisbp;
extern crate tiled;
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use std::fs::File;
use self::sdl2_image::LoadTexture;

pub struct MyLevel {
    pub name: String,
    pub tileset_texture: sdl2::render::Texture,
}

impl ovisbp::Level for MyLevel {
    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }

    fn field(&self, x: usize, y: usize) -> Option<&ovisbp::Field> {
        None
    }

    fn set_field(&self, x: usize, y: usize) -> bool {
        false
    }

    fn start_position(&self) -> (usize, usize) {
        (0,0)
    }

    fn goal_position(&self) -> (usize, usize) {
        (1,0)
    }

    fn jump_height(&self, seconds: f32) -> f32 {
        0.0
    }

    fn player_velocity(&self) -> f32 {
        1.0
    }
}

impl MyLevel {
    pub fn new(name: &str, renderer: &sdl2::render::Renderer) -> MyLevel {
        let file = File::open(&Path::new("data/map.tmx")).unwrap();
        let map = tiled::parse(file).unwrap();
        let mut tileset_texture: Option<sdl2::render::Texture> = None;

        for tileset in map.tilesets.iter() {
            assert!(tileset.images.len() == 1);

            println!("Tileset at {:?}", tileset.images[0].source);

            tileset_texture = Some(renderer.load_texture(&Path::new(&("data/".to_string() + &tileset.images[0].source))).unwrap());
        }

        println!("{:?}", map);

        MyLevel{name: name.to_string(), tileset_texture:tileset_texture.unwrap()}
    }
}
