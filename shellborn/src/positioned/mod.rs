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
