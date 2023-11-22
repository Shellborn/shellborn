use shellborn::relation::builder::RelationBuilder;
use shellborn::relation::cardinality::{Cardinality, FromCardinality, ToCardinality};
use shellborn::relation::connection::Connection;
use shellborn::relation::relation::Relation;

#[test]
fn constructor() {
    assert_eq!(
        Cardinality::zero_to_one(),
        Cardinality {
            from: FromCardinality::Zero,
            to: ToCardinality::One,
        }
    );
    assert_eq!(
        Cardinality::zero_to_many(),
        Cardinality {
            from: FromCardinality::Zero,
            to: ToCardinality::Many,
        }
    );
    assert_eq!(
        Cardinality::one_to_one(),
        Cardinality {
            from: FromCardinality::One,
            to: ToCardinality::One,
        }
    );
    assert_eq!(
        Cardinality::one_to_many(),
        Cardinality {
            from: FromCardinality::One,
            to: ToCardinality::Many,
        }
    );

    assert_eq!(
        Connection::zero_to_one("test"),
        Connection {
            to_name: "test".to_string(),
            cardinalities: Cardinality::zero_to_one(),
        }
    );
    assert_eq!(
        Connection::zero_to_many("test"),
        Connection {
            to_name: "test".to_string(),
            cardinalities: Cardinality::zero_to_many(),
        }
    );
    assert_eq!(
        Connection::one_to_one("test"),
        Connection {
            to_name: "test".to_string(),
            cardinalities: Cardinality::one_to_one(),
        }
    );
    assert_eq!(
        Connection::one_to_many("test"),
        Connection {
            to_name: "test".to_string(),
            cardinalities: Cardinality::one_to_many(),
        }
    )
}

#[test]
fn builder() {
    assert_eq!(
        RelationBuilder::new("test")
            .add_one_to_many("a")
            .add_zero_to_many("b")
            .build(),
        Relation {
            name: "test".to_string(),
            connections: vec![Connection::one_to_many("a"), Connection::zero_to_many("b"),]
        }
    );
}
