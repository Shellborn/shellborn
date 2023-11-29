use crate::entity::Entity;
use crate::render::svg::text::text_width;

#[derive(Default, PartialEq, Debug)]
pub struct Position {
    pub center_x: f32,
    pub center_y: f32,
}

impl Position {
    pub fn set(&mut self, position: (f32, f32)) {
        self.center_x = position.0;
        self.center_y = position.1;
    }
}
