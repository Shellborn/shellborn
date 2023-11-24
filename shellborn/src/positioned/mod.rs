use crate::entity::Entity;
use crate::render::svg::text::text_width;

pub struct Positioned<T> {
    pub data: T,
    pub center_x: f32,
    pub center_y: f32,
}

impl<T> Positioned<T> {
    pub fn new(data: T, center_x: f32, center_y: f32) -> Self {
        Positioned {
            data,
            center_x,
            center_y,
        }
    }
}

impl Positioned<Entity> {
    pub const PADDING: f32 = 2.0;

    pub fn width(&self) -> f32 {
        text_width(&self.data.name) + Self::PADDING * 2.0
    }

    pub fn height(&self) -> f32 {
        10.0 + Self::PADDING * 2.0
    }
}
