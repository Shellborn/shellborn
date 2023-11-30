use crate::attribute::Attribute;
use crate::entity::entity_decoration::EntityDecoration;
use crate::entity::Entity;

pub struct EntityBuilder {
    name: String,
    is_weak: bool,
    attributes: Vec<Attribute>,
}

impl EntityBuilder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            is_weak: false,
            attributes: vec![],
        }
    }

    pub fn set_weak(mut self, is_weak: bool) -> Self {
        self.is_weak = is_weak;
        self
    }

    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn build(self) -> Entity {
        let entity_decoration = if self.is_weak {
            EntityDecoration::Weak
        } else {
            EntityDecoration::Default
        };
        Entity::new(self.name, entity_decoration, self.attributes)
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::Attribute;
    use crate::entity::builder::EntityBuilder;
    use crate::entity::entity_decoration::EntityDecoration;
    use crate::entity::Entity;

    #[test]
    fn builder() {
        assert_eq!(
            EntityBuilder::new("test")
                .set_weak(true)
                .add_attribute(Attribute::basic("attribute"))
                .build(),
            Entity {
                name: "test".to_string(),
                entity_decoration: EntityDecoration::Weak,
                attributes: vec![Attribute::basic("attribute")],
                position: Default::default(),
            }
        )
    }
}
