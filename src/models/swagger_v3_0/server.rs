use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Server Object
///
/// An object representing a Server.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Server {
    /// REQUIRED. A URL to the target host. This URL supports Server Variables and MAY be
    /// relative, to indicate that the host location is relative to the location where the
    /// OpenAPI document is being served. Variable substitutions will be made when a variable
    /// is named in {brackets}.
    pub url: String,
    /// An optional string describing the host designated by the URL. CommonMark syntax MAY be
    /// used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the
    /// server's URL template.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, ServerVariable>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}

/// Server Variable Object
///
/// An object representing a Server Variable for server URL template substitution.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a
    /// limited set. The array SHOULD NOT be empty.
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<String>,
    /// REQUIRED. The default value to use for substitution, which SHALL be sent if an alternate
    /// value is not supplied. Note this behavior is different than the Schema Object's treatment
    /// of default values, because in those cases parameter values are optional. If the enum is
    /// defined, the value SHOULD exist in the enum's values.
    pub default: String,
    /// An optional description for the server variable. CommonMark syntax MAY be used for
    /// rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}