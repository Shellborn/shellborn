use crate::attribute::attribute_decoration::AttributeDecoration;

pub mod attribute_decoration;
pub mod builder;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    decoration: AttributeDecoration,
    sub_attributes: Vec<Attribute>,
}

impl Attribute {
    pub fn basic<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            decoration: AttributeDecoration::Default,
            sub_attributes: vec![],
        }
    }

    pub fn id<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            decoration: AttributeDecoration::Id,
            sub_attributes: vec![],
        }
    }

    pub fn derived<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            decoration: AttributeDecoration::Derived,
            sub_attributes: vec![],
        }
    }

    pub fn multi_valued<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            decoration: AttributeDecoration::MultiValued,
            sub_attributes: vec![],
        }
    }

    pub fn concatenated<S: Into<String>>(
        name: S,
        decoration: AttributeDecoration,
        children: Vec<Attribute>,
    ) -> Self {
        Self {
            name: name.into(),
            decoration,
            sub_attributes: children,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::attribute_decoration::AttributeDecoration;
    use crate::attribute::Attribute;

    #[test]
    fn constructors() {
        assert_eq!(
            Attribute::basic("basic"),
            Attribute {
                name: "basic".to_string(),
                decoration: AttributeDecoration::Default,
                sub_attributes: vec![],
            }
        );

        assert_eq!(
            Attribute::id("id"),
            Attribute {
                name: "id".to_string(),
                decoration: AttributeDecoration::Id,
                sub_attributes: vec![],
            }
        );

        assert_eq!(
            Attribute::derived("derived"),
            Attribute {
                name: "derived".to_string(),
                decoration: AttributeDecoration::Derived,
                sub_attributes: vec![],
            }
        );

        assert_eq!(
            Attribute::multi_valued("multi_valued"),
            Attribute {
                name: "multi_valued".to_string(),
                decoration: AttributeDecoration::MultiValued,
                sub_attributes: vec![],
            }
        );

        assert_eq!(
            Attribute::concatenated(
                "concatenated",
                AttributeDecoration::Default,
                vec![Attribute::basic("test1"), Attribute::basic("test2"),]
            ),
            Attribute {
                name: "concatenated".to_string(),
                decoration: AttributeDecoration::Default,
                sub_attributes: vec![
                    Attribute {
                        name: "test1".to_string(),
                        decoration: AttributeDecoration::Default,
                        sub_attributes: vec![],
                    },
                    Attribute {
                        name: "test2".to_string(),
                        decoration: AttributeDecoration::Default,
                        sub_attributes: vec![],
                    },
                ]
            }
        );
    }
}
