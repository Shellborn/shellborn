#[derive(Debug, PartialEq)]
pub enum AttributeDecoration {
    Default,
    Id,
    MultiValued,
    Derived,
}

impl Default for AttributeDecoration {
    fn default() -> Self {
        Self::Default
    }
}