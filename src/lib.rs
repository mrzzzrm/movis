extern crate ovisbp;
extern crate tiled;
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use std::fs::File;
use self::sdl2_image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Game<'a> {
    pub player: Player,
    pub level: Level,
    pub scale: f32,
    pub sdl: sdl2::Sdl,
    pub renderer: sdl2::render::Renderer<'a>,
    pub event_pump: sdl2::EventPump,
}

pub struct Level {
    pub num_columns: u32,
    pub num_rows: u32,
    pub tiles: Vec<u32>,
    pub tileset: Tileset,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub cell_x: i32,
    pub cell_y: i32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub texture: sdl2::render::Texture,
}

pub struct Tileset {
    pub texture: sdl2::render::Texture,
    pub first_gid: u32,
    pub num_columns: u32,
    pub num_rows: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub spacing: u32,
    pub margin: u32,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        /*
            Init SDL2/SDL2_image
        */
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

        let window = video_subsystem.window("MOvIs", 1600, 900)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        /*
            Level
        */

        let level = Level::new(&renderer);

        /*
            Player
        */
        renderer.set_draw_color(Color::RGBA(255, 255, 0, 0));

        let player_texture = renderer.create_texture(sdl2::pixels::PixelFormatEnum::RGBA8888,
                                                         sdl2::render::TextureAccess::Target,
                                                         (16, 32)).unwrap();
        let _ = renderer.render_target().unwrap().set(player_texture);
        renderer.clear();
        level.tileset.draw_tile(&mut renderer, 1345, 0, 0, 1);
        level.tileset.draw_tile(&mut renderer, 1409, 0, 16, 1);
        let mut player_texture = renderer.render_target().unwrap().reset().unwrap().unwrap();
        player_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        let player = Player::new(10.0, 250.0, player_texture);

        let event_pump = sdl.event_pump().unwrap();

        /*
            Renderer
        */
        renderer.set_draw_color(Color::RGB(128, 128, 128));

        /*

        */
        Game{
            player: player,
            level: level,
            scale: 3.0,
            sdl: sdl,
            renderer: renderer,
            event_pump: event_pump
        }
    }

    pub fn update(&mut self) -> bool {
        /*
            Input
        */
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.player.go_left(true);
                }
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    self.player.go_left(false);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.player.go_right(true);
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    self.player.go_right(false);
                }

                Event::Quit {..} => {
                    return false;
                },
                _ => {}
            }
        }

        /*

        */
        self.player.update();

        /*
            Draw
        */
        self.renderer.clear();

        for r in 0..self.level.num_rows {
            for c in 0..self.level.num_columns {
                self.level.tileset.draw_tile(&mut self.renderer,
                                self.level.tiles[(r * self.level.num_columns + c) as usize],
                                (c as f32 * self.level.tileset.tile_width as f32 * self.scale) as i32,
                                (r as f32 * self.level.tileset.tile_height as f32 * self.scale) as i32,
                                self.scale as u32);
            }
        }

        self.player.update();
        self.player.draw(&mut self.renderer, self.scale);

        self.renderer.present();

        return true;
    }
}

impl Tileset {
    pub fn draw_tile(&self, renderer: &mut sdl2::render::Renderer, gid: u32, x: i32, y: i32, scale: u32) {
        if gid == 0 {
            return;
        }

        assert!(gid >= self.first_gid);

        let lid = gid - self.first_gid;
        let row = lid / self.num_columns;
        let colum = lid % self.num_columns;

        let src = sdl2::rect::Rect::new((self.margin + colum * (self.tile_width + self.spacing)) as i32,
                                        (self.margin + row * (self.tile_height + self.spacing)) as i32,
                                        self.tile_width, self.tile_height).unwrap();
        let dst = sdl2::rect::Rect::new(x, y, self.tile_width * scale, self.tile_height * scale).unwrap();

        renderer.copy(&self.texture, src, dst);
    }
}

impl ovisbp::Level for Level {
    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }

    fn field(&self, _: usize, _: usize) -> Option<&ovisbp::Field> {
        None
    }

    fn set_field(&self, _: usize, _: usize) -> bool {
        false
    }

    fn start_position(&self) -> (usize, usize) {
        (0,0)
    }

    fn goal_position(&self) -> (usize, usize) {
        (1,0)
    }

    fn jump_height(&self, _: f32) -> f32 {
        0.0
    }

    fn player_velocity(&self) -> f32 {
        1.0
    }
}

impl Level {
    pub fn new(renderer: &sdl2::render::Renderer) -> Level {
        let file = File::open(&Path::new("data/map.tmx")).unwrap();
        let map = tiled::parse(file).unwrap();
        let mut tileset = None;

        for ttileset in map.tilesets.iter() {
            assert!(ttileset.images.len() == 1);

            println!("Tileset at {:?}", ttileset.images[0].source);

            let image = &ttileset.images[0];

            let num_columns = (image.width as i32 - ttileset.margin as i32) / (ttileset.tile_width as i32 + ttileset.spacing as i32);
            let num_rows = (image.height as i32- ttileset.margin as i32) / (ttileset.tile_height as i32 + ttileset.spacing as i32);

            println!("Rows: {}, Columns: {}", num_rows, num_columns);

            let tileset_texture = renderer.load_texture(&Path::new(&("data/".to_string() + &image.source))).unwrap();
            tileset = Some(Tileset{texture: tileset_texture,
                                   first_gid: ttileset.first_gid,
                                   num_columns: num_columns as u32,
                                   num_rows: num_rows as u32,
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

        Level{num_columns: map.width,
                num_rows: map.height,
                tiles: tiles,
                tileset:tileset.unwrap()}
    }
}

impl Player {
    pub fn new(x: f32, y: f32, texture: sdl2::render::Texture) -> Player {
        Player {
            x: x,
            y: y,
            cell_x: 0,
            cell_y: 0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            texture: texture,
        }
    }

    pub fn draw(&self, renderer: &mut sdl2::render::Renderer, scale: f32) {
        let src = None;

        let dstw = (16 as f32 * scale) as u32;
        let dsth = (32 as f32 * scale) as u32;
        let dst = sdl2::rect::Rect::new(self.x as i32, self.y as i32, dstw, dsth).unwrap();

        renderer.copy(&self.texture, src, dst);
    }

    pub fn go_left(&mut self, val: bool) {
        if val { self.velocity_x = -0.1; } else { self.velocity_x = 0.0; }
    }

    pub fn go_right(&mut self, val: bool) {
        if val { self.velocity_x = 0.1; } else { self.velocity_x = 0.0; }
    }

    pub fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }
}
