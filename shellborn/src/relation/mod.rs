pub mod builder;
pub mod cardinality;
pub mod connection;

use crate::relation::connection::Connection;

#[derive(PartialEq, Debug)]
pub struct Relation {
    pub name: String,
    pub connections: Vec<Connection>,
}

impl Relation {
    pub fn new<S: Into<String>>(name: S, connections: Vec<Connection>) -> Self {
        Self {
            name: name.into(),
            connections,
        }
    }
}
