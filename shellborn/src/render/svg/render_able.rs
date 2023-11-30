use crate::entity::Entity;
use crate::render::svg::text::{text_element, text_width};
use crate::render::svg::view_box::ViewBox;
use svg::node::element::Rectangle;
use svg::Document;

pub trait SvgRenderAble {
    fn render_onto(&self, document: Document) -> Document;
    fn view_box(&self) -> ViewBox;
}

impl SvgRenderAble for Entity {
    fn render_onto(&self, document: Document) -> Document {
        let width = self.width();
        let height = self.height();

        let body = Rectangle::new()
            .set(
                "x",
                self.position.center_x as i32 - (width / 2.).ceil() as i32,
            )
            .set(
                "y",
                self.position.center_y as i32 - (height / 2.).ceil() as i32,
            )
            .set("width", width)
            .set("height", height);

        document.add(body).add(text_element(
            &self.name,
            self.position.center_x as i32,
            self.position.center_y as i32,
        ))
    }
    fn view_box(&self) -> ViewBox {
        let width = self.width();
        let height = self.height();

        ViewBox::new(
            self.position.center_x as i32 - (width / 2.).ceil() as i32,
            self.position.center_y as i32 - (height / 2.).ceil() as i32,
            self.position.center_x as i32 + (width / 2.).ceil() as i32,
            self.position.center_y as i32 + (height / 2.).ceil() as i32,
        )
    }
}

impl Entity {
    pub const PADDING: f32 = 2.0;

    pub fn width(&self) -> f32 {
        text_width(&self.name) + Self::PADDING * 2.0
    }

    pub fn height(&self) -> f32 {
        10.0 + Self::PADDING * 2.0
    }
}
