extern crate ovisbp;

use ovisbp::*;

struct MyLevel {
    name: String,
}

impl Level for MyLevel {
    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }

    fn field(&self, x: usize, y: usize) -> Option<&Field> {
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
    fn new(name: &str) -> MyLevel {
        MyLevel{name: name.to_string()}
    }
}