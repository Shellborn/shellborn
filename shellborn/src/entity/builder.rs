use crate::attribute::attribute::Attribute;
use crate::attribute::builder::ConcatenatedAttributeBuilder;
use crate::entity::entity::Entity;
use crate::entity::entity_decoration::EntityDecoration;

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

    pub fn add_basic<S: Into<String>>(mut self, name: S) -> Self {
        self.attributes.push(Attribute::basic(name));
        self
    }

    pub fn add_id<S: Into<String>>(mut self, name: S) -> Self {
        self.attributes.push(Attribute::id(name));
        self
    }

    pub fn add_derived<S: Into<String>>(mut self, name: S) -> Self {
        self.attributes.push(Attribute::derived(name));
        self
    }

    pub fn add_multi_valued<S: Into<String>>(mut self, name: S) -> Self {
        self.attributes.push(Attribute::multi_valued(name));
        self
    }

    pub fn add_concatenated(mut self, concatenated: ConcatenatedAttributeBuilder) -> Self {
        self.attributes.push(concatenated.build());
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
