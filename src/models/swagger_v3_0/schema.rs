use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use crate::models::shared::{
    external_documentation::ExternalDocumentation,
    xml::XML,
};
use super::discriminator::Discriminator;

// TODO: documentation
/// Schema Object
///
/// The Schema Object allows the definition of input and output data types. These types can be
/// objects, but also primitives and arrays. This object is an extended subset of the JSON Schema
/// Specification Wright Draft 00.
///
/// For more information about the properties, see JSON Schema Core and JSON Schema Validation.
/// Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Resolvable)]
#[resolve(reference_type = "schema")]
#[serde(default, rename_all = "camelCase")]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<RefOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    /// A true value adds "null" to the allowed type specified by the type keyword, only if type
    /// is explicitly defined within the same Schema Object. Other Schema Object constraints retain
    /// their defined behavior, and therefore may disallow the use of null as a value. A false
    /// value leaves the specified or default type unmodified. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    /// Adds support for polymorphism. The discriminator is an object name that is used to
    /// differentiate between other schemas which may satisfy the payload description. See
    /// Composition and Inheritance for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,
    /// Relevant only for Schema "properties" definitions. Declares the property as "read only".
    /// This means that it MAY be sent as part of a response but SHOULD NOT be sent as part of the
    /// request. If the property is marked as readOnly being true and is in the required list, the
    /// required will take effect on the response only. A property MUST NOT be marked as both
    /// readOnly and writeOnly being true. Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Relevant only for Schema "properties" definitions. Declares the property as "write only".
    /// Therefore, it MAY be sent as part of a request but SHOULD NOT be sent as part of the
    /// response. If the property is marked as writeOnly being true and is in the required list,
    /// the required will take effect on the request only. A property MUST NOT be marked as both
    /// readOnly and writeOnly being true. Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,
    /// This MAY be used only on properties schemas. It has no effect on root schemas. Adds
    /// additional metadata to describe the XML representation of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,
    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A free-form property to include an example of an instance for this schema. To represent
    /// examples that cannot be naturally represented in JSON or YAML, a string value can be used
    /// to contain the example with escaping where necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Specifies that a schema is deprecated and SHOULD be transitioned out of usage. Default
    /// value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}