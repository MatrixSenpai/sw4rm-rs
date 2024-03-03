use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Spec,
    reference::*,
    shared::{
        ParameterLocation,
        Schema,
    },
    openapi_v2::Items,
    openapi_v3_0::{
        Example,
        MediaType,
    },
};

/// Parameter Object
///
/// Describes a single operation parameter.
/// Also implicitly used for `Header`, with `name` as the key of the containing hashmap and
/// `location` being implied as `Header`
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Parameter {
    // MARK: Common Fields

    pub name: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<ParameterLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v2 Fields

    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<ParameterSchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Items>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,

    // MARK: OpenAPI v3 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ParameterStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, RefOr<Example>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
}
impl Resolvable for Parameter {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Parameter => {
               if spec.semver_check("=2") {
                   spec.parameters.get(&reference.name)
                       .ok_or_else(|| ResolveError::UnknownPathError(path))
                       .and_then(|t| t.resolve(spec))
               } else {
                   spec.components
                      .as_ref()
                      .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                      .and_then(|c| c.parameters.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                      .and_then(|p| p.resolve(spec))
               }
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterSchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    File,
    Object,
}
impl Default for ParameterSchemaType {
    fn default() -> Self { Self::String }
}

/// ParameterStyle
///
/// In order to support common ways of serializing simple parameters, a set of style values
/// are defined.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ParameterStyle {
    /// Path-style parameters defined by RFC6570
    Matrix,
    /// Label style parameters defined by RFC6570
    Label,
    /// Form style parameters defined by RFC6570. This option replaces collectionFormat with a
    /// csv (when explode is false) or multi (when explode is true) value from OpenAPI 2.0.
    Form,
    /// Simple style parameters defined by RFC6570. This option replaces collectionFormat with a
    /// csv value from OpenAPI 2.0.
    Simple,
    /// Space separated array values. This option replaces collectionFormat equal to ssv from
    /// OpenAPI 2.0.
    SpaceDelimited,
    /// Pipe separated array values. This option replaces collectionFormat equal to pipes from
    /// OpenAPI 2.0.
    PipeDelimited,
    /// Provides a simple way of rendering nested objects using form parameters.
    DeepObject,
}
impl Default for ParameterStyle {
    fn default() -> Self { Self::Simple }
}