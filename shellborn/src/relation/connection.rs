use crate::relation::cardinality::Cardinality;

#[derive(PartialEq, Debug)]
pub struct Connection {
    pub to_name: String,
    pub cardinalities: Cardinality,
}

impl Connection {
    pub fn zero_to_one<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::zero_to_one(),
        }
    }

    pub fn zero_to_many<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::zero_to_many(),
        }
    }

    pub fn one_to_one<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::one_to_one(),
        }
    }

    pub fn one_to_many<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::one_to_many(),
        }
    }
}