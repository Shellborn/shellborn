#[derive(Default, Debug, PartialEq)]
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

    pub fn from_size(min_x: i32, min_y: i32, width: i32, height: i32) -> ViewBox {
        ViewBox {
            min_x,
            min_y,
            max_x: min_x + width,
            max_y: min_y + height,
        }
    }

    pub fn join(&mut self, other: &ViewBox) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);

        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
    }

    pub fn to_svg_format(&self) -> (i32, i32, i32, i32) {
        (
            self.min_x,
            self.min_y,
            self.max_x - self.min_x,
            self.max_y - self.min_y,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::render::svg::view_box::ViewBox;

    #[test]
    fn from_size() {
        assert_eq!(
            ViewBox::from_size(0, 0, 100, 100),
            ViewBox::new(0, 0, 100, 100)
        );

        assert_eq!(
            ViewBox::from_size(-50, -20, 100, 80),
            ViewBox::new(-50, -20, 50, 60)
        );

        assert_eq!(
            ViewBox::from_size(20, 25, 50, 10),
            ViewBox::new(20, 25, 70, 35)
        );
    }

    #[test]
    fn join() {
        let mut view_box1 = ViewBox::new(0, 0, 0, 0);
        let view_box2 = ViewBox::new(-10, 10, 100, 10);

        view_box1.join(&view_box2);
        assert_eq!(view_box1, ViewBox::new(-10, 0, 100, 10));
    }

    #[test]
    fn to_svg_format() {
        let view_box = ViewBox::new(5, -10, 100, 50);

        assert_eq!(view_box.to_svg_format(), (5, -10, 95, 60));
    }
}
