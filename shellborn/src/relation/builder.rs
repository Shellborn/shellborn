use crate::relation::connection::Connection;
use crate::relation::relation::Relation;

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

    pub fn add_zero_to_one<S: Into<String>>(mut self, to_name: S) -> Self {
        self.connections.push(Connection::zero_to_one(to_name));
        self
    }

    pub fn add_zero_to_many<S: Into<String>>(mut self, to_name: S) -> Self {
        self.connections.push(Connection::zero_to_many(to_name));
        self
    }

    pub fn add_one_to_one<S: Into<String>>(mut self, to_name: S) -> Self {
        self.connections.push(Connection::one_to_one(to_name));
        self
    }

    pub fn add_one_to_many<S: Into<String>>(mut self, to_name: S) -> Self {
        self.connections.push(Connection::one_to_many(to_name));
        self
    }

    pub fn build(self) -> Relation {
        Relation::new(self.name, self.connections)
    }
}