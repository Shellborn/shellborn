#[derive(PartialEq, Debug)]
pub struct Cardinality {
    pub from: FromCardinality,
    pub to: ToCardinality,
}

impl Cardinality {
    pub fn zero_to_one() -> Self {
        Self {
            from: FromCardinality::Zero,
            to: ToCardinality::One,
        }
    }

    pub fn zero_to_many() -> Self {
        Self {
            from: FromCardinality::Zero,
            to: ToCardinality::Many,
        }
    }

    pub fn one_to_one() -> Self {
        Self {
            from: FromCardinality::One,
            to: ToCardinality::One,
        }
    }

    pub fn one_to_many() -> Self {
        Self {
            from: FromCardinality::One,
            to: ToCardinality::Many,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum FromCardinality {
    Zero,
    One,
}

#[derive(PartialEq, Debug)]
pub enum ToCardinality {
    One,
    Many,
}
