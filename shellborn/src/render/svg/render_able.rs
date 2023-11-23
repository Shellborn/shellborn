use crate::entity::Entity;
use crate::positioned::Positioned;
use crate::render::svg::view_box::ViewBox;
use svg::node::element::Ellipse;
use svg::Document;

pub trait SvgRenderAble {
    fn render_onto(&self, document: Document) -> Document;
    fn view_box(&self) -> ViewBox;
}

impl SvgRenderAble for Positioned<Entity> {
    fn render_onto(&self, document: Document) -> Document {
        let width = 100;
        let height = 50;

        let ellipse = Ellipse::new()
            .set("cx", self.center_x as i32)
            .set("cy", self.center_y as i32)
            .set("rx", width / 2)
            .set("ry", height / 2);

        document.add(ellipse)
    }
    fn view_box(&self) -> ViewBox {
        let width = 100;
        let height = 50;

        ViewBox::new(
            self.center_x as i32 - width / 2,
            self.center_y as i32 - height / 2,
            self.center_x as i32 + width / 2,
            self.center_y as i32 + height / 2,
        )
    }
}
