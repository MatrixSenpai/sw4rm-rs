use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    schema::Schema,
    headers::Header,
};

/// Response Object
///
/// Describes a single response from an API Operation.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Resolvable)]
#[resolve(reference_type = "response")]
#[serde(default, rename_all = "camelCase")]
pub struct Response {
    /// Required. A short description of the response. GFM syntax can be used for rich
    /// text representation.
    pub description: String,
    /// A definition of the response structure. It can be a primitive, an array or an object. If
    /// this field does not exist, it means no content is returned as part of the response. As
    /// an extension to the Schema Object, its root type value may also be "file". This SHOULD
    /// be accompanied by a relevant produces mime-type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// A list of headers that are sent with the response.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, Header>,
    /// An example of the response message.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, Value>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
