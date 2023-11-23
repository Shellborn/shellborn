use crate::relation::connection::Connection;
use crate::relation::Relation;

pub struct RelationBuilder {
    name: String,
    connections: Vec<Connection>,
}

impl RelationBuilder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            connections: vec![],
        }
    }

    pub fn add_connection(mut self, connection: Connection) -> Self {
        self.connections.push(connection);
        self
    }

    pub fn build(self) -> Relation {
        Relation::new(self.name, self.connections)
    }
}

#[cfg(test)]
mod tests {
    use crate::relation::builder::RelationBuilder;
    use crate::relation::connection::Connection;
    use crate::relation::Relation;

    #[test]
    fn builder() {
        assert_eq!(
            RelationBuilder::new("test")
                .add_connection(Connection::one_to_many("a"))
                .add_connection(Connection::zero_to_many("b"))
                .build(),
            Relation {
                name: "test".to_string(),
                connections: vec![Connection::one_to_many("a"), Connection::zero_to_many("b"),]
            }
        );
    }
}
