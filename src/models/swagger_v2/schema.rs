use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::shared::{
    external_documentation::ExternalDocumentation,
    xml::XML,
};
use super::{
    parameter::ParameterSchemaType,
};

/// Schema Object
///
/// The Schema Object allows the definition of input and output data types. These types can be
/// objects, but also primitives and arrays. This object is based on the JSON Schema Specification
/// Draft 4 and uses a predefined subset of it. On top of this subset, there are extensions
/// provided by this specification to allow for more complete documentation.
///
/// Further information about the properties can be found in JSON Schema Core and JSON Schema
/// Validation. Unless stated otherwise, the property definitions follow the JSON Schema
/// specification as referenced here.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Schema {
    /// As a JSON Reference.
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    /// See Data Type Formats for further details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Title of item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A description of the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Declares the value of the parameter that the server will use if none is provided, for
    /// example a "count" to control the number of results per page might default to 100 if not
    /// supplied by the client in the request. (Note: "default" has no meaning for required
    /// parameters.) See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-6.2.
    /// Unlike JSON Schema this value MUST conform to the defined type for this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.1.1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.1.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.1.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.1.3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.1.3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.2.1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.2.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.2.3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.3.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.3.3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.3.4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.3.4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.4.1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,
    /// Determines whether this parameter is mandatory. If the parameter is in "path", this
    /// property is required and its value MUST be true. Otherwise, the property MAY be included
    /// and its default value is false.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.4.2.
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Value>,
    /// Required. The type of the parameter. Since the parameter is not located at the request
    /// body, it is limited to simple types (that is, not an object). The value MUST be one of
    /// "string", "number", "integer", "boolean", "array" or "file". If type is "file", the
    /// consumes MUST be either "multipart/form-data", " application/x-www-form-urlencoded" or
    /// both and the parameter MUST be in "formData".
    #[serde(rename = "type")]
    pub schema_type: ParameterSchemaType,
    /// Required if type is "array". Describes the type of items in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.5.3.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<Box<Schema>>,
    /// Properties associated with the schema
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, Box<Schema>>,
    /// Additional properties associated with the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<Schema>>,

    /// Adds support for polymorphism. The discriminator is the schema property name that is
    /// used to differentiate between other schema that inherit this schema. The property name
    /// used MUST be defined at this schema, and it MUST be in the required property list. When
    /// used, the value MUST be the name of this schema or any schema that inherits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    /// Relevant only for Schema "properties" definitions. Declares the property as "read only".
    /// This means that it MAY be sent as part of a response but MUST NOT be sent as part of the
    /// request. Properties marked as readOnly being true SHOULD NOT be in the required list of
    /// the defined schema. Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// This MAY be used only on properties schemas. It has no effect on root schemas. Adds
    /// Additional metadata to describe the XML representation format of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,
    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A free-form property to include an example of an instance for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}