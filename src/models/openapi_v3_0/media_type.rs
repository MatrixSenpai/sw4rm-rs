use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use super::{
    example::Example,
    encoding::Encoding,
    schema::Schema,
};

/// Media Type Object
///
/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<Schema>>,
    /// Example of the media type. The example object SHOULD be in the correct format as specified
    /// by the media type. The example field is mutually exclusive of the examples field.
    /// Furthermore, if referencing a schema which contains an example, the example value SHALL
    /// override the example provided by the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Examples of the media type. Each example object SHOULD match the media type and specified
    /// schema if present. The examples field is mutually exclusive of the example field.
    /// Furthermore, if referencing a schema which contains an example, the examples value SHALL
    /// override the example provided by the schema.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, RefOr<Example>>,
    /// A map between a property name and its encoding information. The key, being the property
    /// name, MUST exist in the schema as a property. The encoding object SHALL only apply to
    /// requestBody objects when the media type is multipart or application/x-www-form-urlencoded.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub encoding: HashMap<String, Encoding>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
