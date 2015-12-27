extern crate ovisbp;
extern crate tiled;
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use std::fs::File;
use self::sdl2_image::LoadTexture;


pub struct Tileset {
    pub texture: sdl2::render::Texture,
    pub first_gid: u32,
    pub numColumns: u32,
    pub numRows: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub spacing: u32,
    pub margin: u32,
}

impl Tileset {
    pub fn draw_tile(&self, renderer: &mut sdl2::render::Renderer, gid: u32, x: i32, y: i32, scale: u32) {
        if gid == 0 {
            return;
        }

        assert!(gid >= self.first_gid);

        let lid = gid - self.first_gid;
        let row = lid / self.numColumns;
        let colum = lid % self.numColumns;

        let src = sdl2::rect::Rect::new((self.margin + colum * (self.tile_width + self.spacing)) as i32,
                                        (self.margin + row * (self.tile_height + self.spacing)) as i32,
                                        self.tile_width, self.tile_height).unwrap();
        let dst = sdl2::rect::Rect::new(x, y, self.tile_width * scale, self.tile_height * scale).unwrap();

        renderer.copy(&self.texture, src, dst);
    }
}

pub struct MyLevel {
    pub numColumns: u32,
    pub numRows: u32,
    pub tiles: Vec<u32>,
    pub tileset: Tileset,
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
    pub fn new(renderer: &sdl2::render::Renderer) -> MyLevel {
        let file = File::open(&Path::new("data/map.tmx")).unwrap();
        let map = tiled::parse(file).unwrap();
        let mut tileset = None;

        for ttileset in map.tilesets.iter() {
            assert!(ttileset.images.len() == 1);

            println!("Tileset at {:?}", ttileset.images[0].source);

            let image = &ttileset.images[0];

            let numColumns = (image.width as i32 - ttileset.margin as i32) / (ttileset.tile_width as i32 + ttileset.spacing as i32);
            let numRows = (image.height as i32- ttileset.margin as i32) / (ttileset.tile_height as i32 + ttileset.spacing as i32);

            println!("Rows: {}, Columns: {}", numRows, numColumns);

            let tileset_texture = renderer.load_texture(&Path::new(&("data/".to_string() + &image.source))).unwrap();
            tileset = Some(Tileset{texture: tileset_texture,
                                   first_gid: ttileset.first_gid,
                                   numColumns: numColumns as u32,
                                   numRows: numRows as u32,
                                   tile_width: ttileset.tile_width,
                                   tile_height: ttileset.tile_height,
                                   spacing: ttileset.spacing,
                                   margin: ttileset.margin});
        }

        println!("{:?}", map);

        assert!(map.layers.len() > 0);
        let layer = &map.layers[0];
        let mut tiles = Vec::<u32>::with_capacity((map.width * map.height) as usize);

        for r in 0..map.height {
            for c in 0..map.width {
                tiles.push(layer.tiles[r as usize][c as usize]);
            }
        }

        MyLevel{numColumns: map.width,
                numRows: map.height,
                tiles: tiles,
                tileset:tileset.unwrap()}
    }
}
