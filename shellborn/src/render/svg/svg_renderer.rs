use crate::render::svg::view_box::ViewBox;

use crate::entity::Entity;
use crate::positioned::Positioned;
use std::path::Path;
use svg::node::element::Ellipse;
use svg::Document;

pub struct SVGRenderer {
    view_box: ViewBox,
    document: Document,
}

impl Default for SVGRenderer {
    fn default() -> Self {
        SVGRenderer {
            view_box: ViewBox::default(),
            document: Document::new(),
        }
    }
}

impl SVGRenderer {
    pub fn add_entity(&mut self, entity: &Positioned<Entity>) {
        let width = 100;
        let height = 50;

        self.update_view_box(&ViewBox::new(
            entity.center_x as i32 - width / 2,
            entity.center_y as i32 - height / 2,
            entity.center_x as i32 + width / 2,
            entity.center_y as i32 + height / 2,
        ));

        let ellipse = Ellipse::new()
            .set("cx", entity.center_x as i32)
            .set("cy", entity.center_y as i32)
            .set("rx", width / 2)
            .set("ry", height / 2);

        self.document = self.document.to_owned().add(ellipse);
    }

    fn update_view_box(&mut self, other_view_box: &ViewBox) {
        self.view_box.join(other_view_box);
        self.document = self
            .document
            .to_owned()
            .set("viewBox", self.view_box.to_svg_format());
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        svg::save(path, &self.document).unwrap();
    }
}
