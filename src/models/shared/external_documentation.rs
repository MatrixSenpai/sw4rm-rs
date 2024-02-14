use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// External Documentation Object
///
/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ExternalDocumentation {
    /// A short description of the target documentation. GFM syntax can be used for rich text
    /// representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required. The URL for the target documentation. Value MUST be in the format of a URL.
    pub url: String,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
