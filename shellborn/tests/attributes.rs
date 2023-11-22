use shellborn::attribute::attribute::Attribute;
use shellborn::attribute::attribute_decoration::AttributeDecoration;
use shellborn::attribute::builder::ConcatenatedAttributeBuilder;

#[test]
fn constructors() {
    assert_eq!(
        Attribute::basic("basic"),
        Attribute::Single {
            name: "basic".to_string(),
            decoration: AttributeDecoration::Default
        }
    );

    assert_eq!(
        Attribute::id("id"),
        Attribute::Single {
            name: "id".to_string(),
            decoration: AttributeDecoration::Id
        }
    );

    assert_eq!(
        Attribute::derived("derived"),
        Attribute::Single {
            name: "derived".to_string(),
            decoration: AttributeDecoration::Derived
        }
    );

    assert_eq!(
        Attribute::multi_valued("multi_valued"),
        Attribute::Single {
            name: "multi_valued".to_string(),
            decoration: AttributeDecoration::MultiValued
        }
    );

    assert_eq!(
        Attribute::concatenated(
            "concatenated",
            vec![Attribute::basic("test1"), Attribute::basic("test2"),]
        ),
        Attribute::Concatenated {
            name: "concatenated".to_string(),
            sub_attributes: vec![
                Attribute::Single {
                    name: "test1".to_string(),
                    decoration: AttributeDecoration::Default
                },
                Attribute::Single {
                    name: "test2".to_string(),
                    decoration: AttributeDecoration::Default
                },
            ]
        }
    );
}

#[test]
fn builder() {
    assert_eq!(
        ConcatenatedAttributeBuilder::new("concatenated")
            .add_basic("basic")
            .add_concatenated(
                ConcatenatedAttributeBuilder::new("sub_concatenated").add_basic("sub_basic"),
            )
            .build(),
        Attribute::Concatenated {
            name: "concatenated".to_string(),
            sub_attributes: vec![
                Attribute::Single {
                    name: "basic".to_string(),
                    decoration: AttributeDecoration::Default
                },
                Attribute::Concatenated {
                    name: "sub_concatenated".to_string(),
                    sub_attributes: vec![Attribute::Single {
                        name: "sub_basic".to_string(),
                        decoration: AttributeDecoration::Default
                    }]
                }
            ]
        }
    );
}
