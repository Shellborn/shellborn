use shellborn::entity::entity_decoration::EntityDecoration;
use shellborn::entity::Entity;
use shellborn::positioned::Positioned;
use shellborn::render::svg::svg_renderer::SVGRenderer;

fn main() {
    let mut svg = SVGRenderer::default();

    Entity::new("huff", EntityDecoration::Default, vec![]);

    svg.add_entity(&Positioned::new(
        Entity::new("huff", EntityDecoration::Default, vec![]),
        10.0,
        100.0,
    ));

    svg.add_entity(&Positioned::new(
        Entity::new("puff", EntityDecoration::Default, vec![]),
        -20.0,
        0.0,
    ));

    svg.save("examples/ellipse.svg");
}
