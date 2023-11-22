use crate::attribute::attribute::Attribute;
use crate::entity::entity_decoration::EntityDecoration;

#[derive(PartialEq, Debug)]
pub struct Entity {
    pub name: String,
    pub entity_decoration: EntityDecoration,
    pub attributes: Vec<Attribute>,
}

impl Entity {
    pub fn new<S: Into<String>>(name: S, entity_decoration: EntityDecoration, attributes: Vec<Attribute>) -> Self {
        Self { name: name.into(), entity_decoration, attributes }
    }
}