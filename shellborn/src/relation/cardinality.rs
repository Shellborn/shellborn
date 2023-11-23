#[derive(PartialEq, Debug)]
pub struct Cardinality {
    pub from: String,
    pub to: String,
}

impl Cardinality {
    pub fn new<S: Into<String>>(from: S, to: S) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}
