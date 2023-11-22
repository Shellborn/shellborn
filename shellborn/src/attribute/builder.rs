use crate::attribute::attribute_decoration::AttributeDecoration;
use crate::attribute::Attribute;

pub struct ConcatenatedAttributeBuilder {
    name: String,
    decoration: AttributeDecoration,
    sub_attributes: Vec<Attribute>,
}

impl ConcatenatedAttributeBuilder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            decoration: AttributeDecoration::Default,
            sub_attributes: vec![],
        }
    }

    pub fn set_decoration(mut self, decoration: AttributeDecoration) -> Self {
        self.decoration = decoration;
        self
    }

    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.sub_attributes.push(attribute);
        self
    }

    pub fn build(self) -> Attribute {
        Attribute {
            name: self.name,
            decoration: self.decoration,
            sub_attributes: self.sub_attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::attribute_decoration::AttributeDecoration;
    use crate::attribute::builder::ConcatenatedAttributeBuilder;
    use crate::attribute::Attribute;

    #[test]
    fn builder() {
        assert_eq!(
            ConcatenatedAttributeBuilder::new("concatenated")
                .add_attribute(Attribute::basic("basic"))
                .add_attribute(
                    ConcatenatedAttributeBuilder::new("sub_concatenated")
                        .add_attribute(Attribute::basic("sub_basic"))
                        .build(),
                )
                .build(),
            Attribute {
                name: "concatenated".to_string(),
                decoration: AttributeDecoration::Default,
                sub_attributes: vec![
                    Attribute {
                        name: "basic".to_string(),
                        decoration: AttributeDecoration::Default,
                        sub_attributes: vec![],
                    },
                    Attribute {
                        name: "sub_concatenated".to_string(),
                        decoration: AttributeDecoration::Default,
                        sub_attributes: vec![Attribute {
                            name: "sub_basic".to_string(),
                            decoration: AttributeDecoration::Default,
                            sub_attributes: vec![],
                        }]
                    }
                ]
            }
        );
    }
}
