use shellborn::entity::entity_decoration::EntityDecoration;
use shellborn::entity::Entity;
use shellborn::render::svg::svg_renderer::SVGRenderer;

fn main() {
    let mut svg = SVGRenderer::default();

    let mut entity = Entity::new("House", EntityDecoration::Default, vec![]);
    entity.position.set((12.0, 20.0));
    svg.append(&entity);

    let mut entity = Entity::new("Garage", EntityDecoration::Default, vec![]);
    entity.position.set((-20.0, -30.0));
    svg.append(&entity);

    let mut entity = Entity::new("Building", EntityDecoration::Default, vec![]);
    entity.position.set((-30.0, 0.0));
    svg.append(&entity);

    svg.save("examples/entity.svg");
}
