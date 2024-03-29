use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::shared::ParameterSchemaType;

/// Items Object
///
/// A limited subset of JSON-Schema's items object. It is used by parameter definitions that
/// are not located in "body".
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Items {
    /// Required. The type of the parameter. Since the parameter is not located at the request
    /// body, it is limited to simple types (that is, not an object). The value MUST be one of
    /// "string", "number", "integer", "boolean", "array" or "file". If type is "file", the
    /// consumes MUST be either "multipart/form-data", " application/x-www-form-urlencoded" or
    /// both and the parameter MUST be in "formData".
    #[serde(rename = "schema")]
    pub schema_type: ParameterSchemaType,
    /// The extending format for the previously mentioned type. See Data Type Formats for
    /// further details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    // Note: This type may be recursive, so it is boxed
    /// Required if type is "array". Describes the type of items in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Items>>,
    /// Determines the format of the array if type array is used. Possible values are:
    ///
    /// - csv - comma separated values foo,bar.
    /// - ssv - space separated values foo bar.
    /// - tsv - tab separated values foo\tbar.
    /// - pipes - pipe separated values foo|bar.
    /// - multi - corresponds to multiple parameter instances instead of multiple values
    ///  for a single instance foo=bar&foo=baz. This is valid only for parameters in "query" or
    ///  "formData".
    ///
    /// Default value is csv.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<bool>,
    /// Declares the value of the parameter that the server will use if none is provided, for
    /// example a "count" to control the number of results per page might default to 100 if not
    /// supplied by the client in the request. (Note: "default" has no meaning for required
    /// parameters.) See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-6.2.
    /// Unlike JSON Schema this value MUST conform to the defined type for this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
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
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.5.1.
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Value>,
    /// See https://tools.ietf.org/html/draft-fge-json-schema-validation-00#section-5.1.1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
