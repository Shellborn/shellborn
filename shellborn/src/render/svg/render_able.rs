use crate::entity::Entity;
use crate::positioned::Positioned;
use crate::render::svg::text::text_element;
use crate::render::svg::view_box::ViewBox;
use svg::node::element::Rectangle;
use svg::Document;

pub trait SvgRenderAble {
    fn render_onto(&self, document: Document) -> Document;
    fn view_box(&self) -> ViewBox;
}

impl SvgRenderAble for Positioned<Entity> {
    fn render_onto(&self, document: Document) -> Document {
        let width = self.width();
        let height = self.height();

        let body = Rectangle::new()
            .set("x", self.center_x as i32 - (width / 2.).ceil() as i32)
            .set("y", self.center_y as i32 - (height / 2.).ceil() as i32)
            .set("width", width)
            .set("height", height);

        document.add(body).add(text_element(
            &self.data.name,
            self.center_x as i32,
            self.center_y as i32,
        ))
    }
    fn view_box(&self) -> ViewBox {
        let width = self.width();
        let height = self.height();

        ViewBox::new(
            self.center_x as i32 - (width / 2.).ceil() as i32,
            self.center_y as i32 - (height / 2.).ceil() as i32,
            self.center_x as i32 + (width / 2.).ceil() as i32,
            self.center_y as i32 + (height / 2.).ceil() as i32,
        )
    }
}
