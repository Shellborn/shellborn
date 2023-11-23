use crate::relation::cardinality::Cardinality;

#[derive(PartialEq, Debug)]
pub struct Connection {
    pub to_name: String,
    pub cardinalities: Cardinality,
}

impl Connection {
    pub fn new<S: Into<String>>(to_name: S, from: S, to: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::new(from, to),
        }
    }

    pub fn zero_to_one<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::new("0", "1"),
        }
    }

    pub fn zero_to_many<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::new("0", "n"),
        }
    }

    pub fn one_to_one<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::new("1", "1"),
        }
    }

    pub fn one_to_many<S: Into<String>>(to_name: S) -> Self {
        Self {
            to_name: to_name.into(),
            cardinalities: Cardinality::new("1", "n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::relation::cardinality::Cardinality;
    use crate::relation::connection::Connection;

    #[test]
    fn constructor() {
        assert_eq!(
            Connection::zero_to_one("test"),
            Connection {
                to_name: "test".to_string(),
                cardinalities: Cardinality::new("0", "1"),
            }
        );
        assert_eq!(
            Connection::zero_to_many("test"),
            Connection {
                to_name: "test".to_string(),
                cardinalities: Cardinality::new("0", "n"),
            }
        );
        assert_eq!(
            Connection::one_to_one("test"),
            Connection {
                to_name: "test".to_string(),
                cardinalities: Cardinality::new("1", "1"),
            }
        );
        assert_eq!(
            Connection::one_to_many("test"),
            Connection {
                to_name: "test".to_string(),
                cardinalities: Cardinality::new("1", "n"),
            }
        );
    }
}
