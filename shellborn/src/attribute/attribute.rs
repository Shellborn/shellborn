use crate::attribute::attribute::Attribute::{Concatenated, Single};
use crate::attribute::attribute_decoration::AttributeDecoration;

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Single {
        name: String,
        decoration: AttributeDecoration,
    },
    Concatenated {
        name: String,
        sub_attributes: Vec<Attribute>,
    },
}

impl Attribute {
    pub fn basic<S: Into<String>>(name: S) -> Self {
        Single {
            name: name.into(),
            decoration: AttributeDecoration::Default,
        }
    }

    pub fn id<S: Into<String>>(name: S) -> Self {
        Single {
            name: name.into(),
            decoration: AttributeDecoration::Id,
        }
    }

    pub fn derived<S: Into<String>>(name: S) -> Self {
        Single {
            name: name.into(),
            decoration: AttributeDecoration::Derived,
        }
    }

    pub fn multi_valued<S: Into<String>>(name: S) -> Self {
        Single {
            name: name.into(),
            decoration: AttributeDecoration::MultiValued,
        }
    }

    pub fn concatenated<S: Into<String>>(name: S, children: Vec<Attribute>) -> Self {
        Concatenated {
            name: name.into(),
            sub_attributes: children,
        }
    }
}
