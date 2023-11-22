use shellborn::attribute::attribute::Attribute;
use shellborn::entity::builder::EntityBuilder;
use shellborn::entity::entity::Entity;
use shellborn::entity::entity_decoration::EntityDecoration;

#[test]
fn constructor() {
    assert_eq!(
        Entity::new(
            "test",
            EntityDecoration::Default,
            vec![Attribute::basic("attribute")],
        ),
        Entity {
            name: "test".to_string(),
            entity_decoration: EntityDecoration::Default,
            attributes: vec![Attribute::basic("attribute")]
        }
    );
}

#[test]
fn builder() {
    assert_eq!(
        EntityBuilder::new("test")
            .set_weak(true)
            .add_basic("attribute")
            .build(),
        Entity {
            name: "test".to_string(),
            entity_decoration: EntityDecoration::Weak,
            attributes: vec![Attribute::basic("attribute")]
        }
    )
}
