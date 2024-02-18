use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Discriminator Object
///
/// When request bodies or response payloads may be one of a number of different schemas, a
/// discriminator object can be used to aid in serialization, deserialization, and validation. The
/// discriminator is a specific object in a schema which is used to inform the consumer of the
/// specification of an alternative schema based on the value associated with it.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Discriminator {
    /// REQUIRED. The name of the property in the payload that will hold the discriminator value.
    pub property_name: String,
    /// An object to hold mappings between payload values and schema names or references.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub mapping: HashMap<String, String>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}