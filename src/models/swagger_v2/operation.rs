use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use crate::models::{
    Scheme, StringOrHttpCode,
    shared::external_documentation::ExternalDocumentation,
};
use super::{
    parameter::Parameter,
    response::Response,
};

/// Operation Object
///
/// Describes a single API operation on a path.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of
    /// operations by resources or any other qualifier.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// A short summary of what the operation does. For maximum readability in the swagger-ui,
    /// this field SHOULD be less than 120 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. GFM syntax can be used for rich text
    /// representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations
    /// described in the API. Tools and libraries MAY use the operationId to uniquely identify
    /// an operation, therefore, it is recommended to follow common programming naming conventions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A list of MIME types the operation can consume. This overrides the consumes definition
    /// at the Swagger Object. An empty value MAY be used to clear the global definition. Value
    /// MUST be as described under Mime Types.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    /// A list of MIME types the operation can produce. This overrides the produces definition
    /// at the Swagger Object. An empty value MAY be used to clear the global definition. Value
    /// MUST be as described under Mime Types.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    /// A list of parameters that are applicable for this operation. If a parameter is already
    /// defined at the Path Item, the new definition will override it, but can never remove it.
    /// The list MUST NOT include duplicated parameters. A unique parameter is defined by a
    /// combination of a name and location. The list can use the Reference Object to link to
    /// parameters that are defined at the Swagger Object's parameters. There can be one "body"
    /// parameter at most.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
    /// Required. The list of possible responses as they are returned from executing this
    /// operation.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<StringOrHttpCode, RefOr<Response>>,
    /// The transfer protocol for the operation. Values MUST be from the list: "http", "https",
    /// "ws", "wss". The value overrides the Swagger Object schemes definition.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<Scheme>,
    /// Declares this operation to be deprecated. Usage of the declared operation should be
    /// refrained. Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// A declaration of which security schemes are applied for this operation. The list of
    /// values describes alternative security schemes that can be used (that is, there is a
    /// logical OR between the security requirements). This definition overrides any declared
    /// top-level security. To remove a top-level security declaration, an empty array can be
    /// used.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
