use crate::render::svg::view_box::ViewBox;

use crate::render::svg::render_able::SvgRenderAble;
use std::path::Path;

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
    pub fn append(&mut self, render_able: &dyn SvgRenderAble) {
        self.update_view_box(&render_able.view_box());
        self.document = render_able.render_onto(self.document.to_owned());
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
