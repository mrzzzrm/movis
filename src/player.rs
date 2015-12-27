extern crate sdl2;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub texture: sdl2::render::Texture,
}

impl Player {
    pub fn draw(&self, renderer: &mut sdl2::render::Renderer) {
        let src = None;
        let dst = sdl2::rect::Rect::new(self.x as i32, self.y as i32, 16, 32).unwrap();

        renderer.copy(&self.texture, src, dst);
    }

    pub fn update(&mut self) {
        self.y += 0.1;        
    }
}
