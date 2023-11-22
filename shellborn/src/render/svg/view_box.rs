#[derive(Default)]
pub struct ViewBox {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl ViewBox {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> ViewBox {
        ViewBox {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn join(&mut self, other: &ViewBox) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);

        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
    }

    /// returns the viewBox in (min_x, min_y, width, height)
    pub fn to_svg_format(&self) -> (i32, i32, i32, i32) {
        (
            self.min_x,
            self.min_y,
            self.max_x - self.min_x,
            self.max_y - self.min_y,
        )
    }
}
