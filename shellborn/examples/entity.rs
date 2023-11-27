use shellborn::entity::entity_decoration::EntityDecoration;
use shellborn::entity::Entity;
use shellborn::positioned::Positioned;
use shellborn::render::svg::svg_renderer::SVGRenderer;

fn main() {
    let mut svg = SVGRenderer::default();

    svg.append(&Positioned::new(
        Entity::new("House", EntityDecoration::Default, vec![]),
        10.0,
        20.0,
    ));

    svg.append(&Positioned::new(
        Entity::new("Garage", EntityDecoration::Default, vec![]),
        -20.0,
        -30.0,
    ));

    svg.append(&Positioned::new(
        Entity::new("Building", EntityDecoration::Default, vec![]),
        -30.0,
        0.0,
    ));

    svg.save("examples/entity.svg");
}
