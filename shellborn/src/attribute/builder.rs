use crate::attribute::attribute::Attribute;

pub struct ConcatenatedAttributeBuilder {
    name: String,
    sub_attributes: Vec<Attribute>,
}

impl ConcatenatedAttributeBuilder {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            sub_attributes: vec![],
        }
    }

    pub fn add_basic<S: Into<String>>(mut self, name: S) -> Self {
        self.sub_attributes.push(Attribute::basic(name));
        self
    }

    pub fn add_id<S: Into<String>>(mut self, name: S) -> Self {
        self.sub_attributes.push(Attribute::id(name));
        self
    }

    pub fn add_derived<S: Into<String>>(mut self, name: S) -> Self {
        self.sub_attributes.push(Attribute::derived(name));
        self
    }

    pub fn add_multi_valued<S: Into<String>>(mut self, name: S) -> Self {
        self.sub_attributes.push(Attribute::multi_valued(name));
        self
    }

    pub fn add_concatenated(mut self, concatenated: ConcatenatedAttributeBuilder) -> Self {
        self.sub_attributes.push(concatenated.build());
        self
    }

    pub fn build(self) -> Attribute {
        Attribute::concatenated(self.name, self.sub_attributes)
    }
}