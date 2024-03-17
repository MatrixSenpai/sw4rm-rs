use std::sync::Arc;
use std::collections::HashMap;
use sw4rm_rs::{
    shared::{Schema, SchemaType, SchemaTypeContainer}, Spec
};
use super::{
    attribute::Attribute, error::GenerationError, identifier::Identifier, import::Import
};

/// A field in a struct or enum
#[derive(Debug, Clone)]
pub struct Field {
    /// If the type needs to be exported, it is easier to define it here than trying to pass it all
    /// the way up. Should be collected by `Model`
    pub imports: Vec<Import>,
    /// A documentation attribute
    pub documentation: Option<String>,
    /// Any attributes for the field. E.g. `serde(skip_serializing)`
    pub attributes: Vec<Attribute>,
    /// The field name. Should be `Ident` ready
    pub field_name: Identifier,
    /// The field type
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub struct FieldInputParams {
    pub definition_key: String,
    pub field: Box<Schema>,
    pub spec: Arc<Spec>,
}

impl TryFrom<FieldInputParams> for Field {
    type Error = GenerationError;

    fn try_from(value: FieldInputParams) -> Result<Self, Self::Error> {
        let field_name = value.field.title.clone()
            .unwrap_or(value.definition_key.clone());

        Ok(
            Self {
                // TODO: do imports
                imports: Vec::new(),
                documentation: value.field.description.clone(),
                // TODO: do attributes
                attributes: Vec::new(),
                field_name: field_name.into(),
                field_type: value.clone().try_into()?,
            }
        )
    }
}

/// Either a concrete or generic type
#[derive(Debug, Clone)]
pub enum FieldType {
    /// A concrete type. E.g. `String`, `impl DeserializeOwned`
    Concrete(FieldConcreteType),
    /// A generic field type. E.g. `Option<String>`
    Generic(FieldConcreteType, Box<FieldType>),
    /// A generic field type that specifies a key and value. E.g. `HashMap<String, String>`
    GenericPair(FieldConcreteType, Box<FieldType>, Box<FieldType>)
}

#[derive(Debug, Clone)]
pub enum FieldConcreteType {
    Array,
    Map,
    Optional,
    Bool,
    Integer,
    Float,
    String,
}

impl TryFrom<FieldInputParams> for FieldType {
    type Error = GenerationError;

    fn try_from(value: FieldInputParams) -> Result<Self, Self::Error> {
        let kind = match value.field.schema_type {
            Some(SchemaTypeContainer::SingleType(v)) => v,
            Some(SchemaTypeContainer::MultiType(v)) => v.first().unwrap().to_owned(),
            _ => return Err(GenerationError::MissingType)
        };

        let kind = match kind {
            SchemaType::Boolean => FieldConcreteType::Bool,
            SchemaType::String => FieldConcreteType::String,
            SchemaType::Integer => FieldConcreteType::Integer,
            SchemaType::Number => FieldConcreteType::Float,

            _ => return Err(Self::Error::Incomplete),
        };
        
        Ok(FieldType::Concrete(kind))
    }
}
