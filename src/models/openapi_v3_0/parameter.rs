use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use crate::models::shared::parameter_location::ParameterLocation;
use super::{
    example::Example,
    media_type::MediaType,
    schema::Schema,
};

/// Parameter Object
///
/// Describes a single operation parameter.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Resolvable)]
#[resolve(reference_type = "parameter")]
#[serde(default, rename_all = "camelCase")]
pub struct Parameter {
    /// REQUIRED. The name of the parameter. Parameter names are case-sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// REQUIRED. The location of the parameter. Possible values are "query", "header", "path"
    /// or "cookie".
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub location: Option<ParameterLocation>,
    /// A brief description of the parameter. This could contain examples of use. CommonMark syntax
    /// MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory. If the parameter location is "path", this
    /// property is REQUIRED and its value MUST be true. Otherwise, the property MAY be included
    /// and its default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage. Default
    /// value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters. This is valid only for query parameters
    /// and allows sending a parameter with an empty value. Default value is false. If style is
    /// used, and if behavior is n/a (cannot be serialized), the value of allowEmptyValue SHALL be
    /// ignored. Use of this property is NOT RECOMMENDED, as it is likely to be removed in a later
    /// revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
    /// Describes how the parameter value will be serialized depending on the type of the parameter
    /// value. Default values (based on value of in): for query - form; for path - simple; for
    /// header - simple; for cookie - form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ParameterStyle>,
    /// When this is true, parameter values of type array or object generate separate parameters
    /// for each value of the array or key-value pair of the map. For other types of parameters
    /// this property has no effect. When style is form, the default value is true. For all other
    /// styles, the default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by
    /// RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding. This property only
    /// applies to parameters with an in value of query. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,
    /// The schema defining the type used for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<Schema>>,
    /// Example of the parameter's potential value. The example SHOULD match the specified schema
    /// and encoding properties if present. The example field is mutually exclusive of the examples
    /// field. Furthermore, if referencing a schema that contains an example, the example value
    /// SHALL override the example provided by the schema. To represent examples of media types
    /// that cannot naturally be represented in JSON or YAML, a string value can contain the
    /// example with escaping where necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    /// Examples of the parameter's potential value. Each example SHOULD contain a value in the
    /// correct format as specified in the parameter encoding. The examples field is mutually
    /// exclusive of the example field. Furthermore, if referencing a schema that contains an
    /// example, the examples value SHALL override the example provided by the schema.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, RefOr<Example>>,
    /// A map containing the representations for the parameter. The key is the media type and the
    /// value describes it. The map MUST only contain one entry.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
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
