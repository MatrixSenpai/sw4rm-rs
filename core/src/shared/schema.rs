use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Spec,
    reference::*,
    shared::{
        ExternalDocumentation,
        StringOrDiscriminator,
        XML,
    },
};

/// Schema Object
///
/// The Schema Object allows the definition of input and output data types. These types can be
/// objects, but also primitives and arrays. This object is an extended subset of the JSON Schema
/// Specification Wright Draft 00.
///
/// For more information about the properties, see JSON Schema Core and JSON Schema Validation.
/// Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Schema {
    // MARK: Common Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<SchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<StringOrDiscriminator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v3 Fields

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

impl Resolvable for Schema {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Schema | ReferenceType::Definitions => {
                if spec.semver_check("=2") {
                    spec.definitions.get(&reference.name)
                        .ok_or_else(|| ResolveError::UnknownPathError(path))
                        .and_then(|t| t.resolve(spec))
                } else {
                    spec.components
                        .as_ref()
                        .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                        .and_then(|c| c.schemas.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                        .and_then(|p| p.resolve(spec))
                }
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SchemaType {
    Array,
    Boolean,
    File,
    Integer,
    Number,
    Object,
    String,
}
impl Default for SchemaType {
    fn default() -> Self { Self::Object }
}