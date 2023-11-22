use shellborn::forms::entity_type::EntityType;
use shellborn::render::svg::svg_renderer::SVGRenderer;

fn main() {
    let mut svg = SVGRenderer::default();

    svg.add_entity_type(&EntityType {
        name: "test".to_string(),
        center_x: 10,
        center_y: 100,
    });

    svg.add_entity_type(&EntityType {
        name: "test".to_string(),
        center_x: -20,
        center_y: 0,
    });

    svg.save("examples/ellipse.svg");
}
