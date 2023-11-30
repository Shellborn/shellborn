pub mod builder;
pub mod entity_decoration;

use crate::attribute::Attribute;
use crate::entity::entity_decoration::EntityDecoration;
use crate::position::Position;

#[derive(PartialEq, Debug)]
pub struct Entity {
    pub name: String,
    pub entity_decoration: EntityDecoration,
    pub attributes: Vec<Attribute>,
    pub position: Position,
}

impl Entity {
    pub fn new<S: Into<String>>(
        name: S,
        entity_decoration: EntityDecoration,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            name: name.into(),
            entity_decoration,
            attributes,
            position: Position::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::Attribute;
    use crate::entity::entity_decoration::EntityDecoration;
    use crate::entity::Entity;

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
                attributes: vec![Attribute::basic("attribute")],
                position: Default::default(),
            }
        );
    }
}
