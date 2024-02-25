use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::shared::ExternalDocumentation;

/// Tag Object
///
/// Allows adding metadata to a single tag that is used by the Operation Object. It is not
/// mandatory to have a Tag Object per tag used there.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Tag {
    /// Required. The name of the tag.
    pub name: String,
    /// A short description for the tag. GFM syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
