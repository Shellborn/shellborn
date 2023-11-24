use svg::node;
use svg::node::element;

pub fn text_width(text: &str) -> f32 {
    10.0 * 0.55 * text.len() as f32
}
pub fn text_element(text: &str, center_x: i32, center_y: i32) -> element::Text {
    let text_node = node::Text::new(text);
    element::Text::new()
        .add(text_node)
        .set("x", center_x)
        .set("y", center_y)
        .set("dx", -text_width(text) / 2.)
        .set("dy", 10. / 3.)
        .set("fill", "white")
        .set("font-family", "monospace")
        .set("font-size", 10.0)
}
