use super::{
    import::Import,
    attribute::Attribute,
};

/// A field in a struct or enum
#[derive(Debug, Clone)]
pub struct Field {
    /// If the type needs to be exported, it is easier to define it here than trying to pass it all
    /// the way up. Should be collected by `Model`
    pub imports: Vec<Import>,
    /// A documentation attribute
    pub documentation: String,
    /// Any attributes for the field. E.g. `serde(skip_serializing)`
    pub attributes: Vec<Attribute>,
    /// The field name. Should be `Ident` ready
    pub field_name: String,
    /// The field type
    pub field_type: FieldType,
}

/// Either a concrete or generic type
#[derive(Debug, Clone)]
pub enum FieldType {
    /// A concrete type. E.g. `String`, `impl DeserializeOwned`
    Concrete(String),
    /// A generic field type. E.g. `Option<String>`
    Generic(String, Box<FieldType>),
    /// A generic field type that specifies a key and value. E.g. `HashMap<String, String>`
    GenericPair(String, Box<FieldType>, Box<FieldType>)
}
