use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use crate::models::{
    StringOrHttpCode,
    shared::external_documentation::ExternalDocumentation,
};
use super::{
    parameter::Parameter,
    request_body::RequestBody,
    response::Response,
    path::PathItem,
    server::Server,
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
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. CommonMark syntax MAY be used for rich
    /// text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations
    /// described in the API. The operationId value is case-sensitive. Tools and libraries MAY use
    /// the operationId to uniquely identify an operation, therefore, it is RECOMMENDED to follow
    /// common programming naming conventions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation. If a parameter is already
    /// defined at the Path Item, the new definition will override it but can never remove it.
    /// The list MUST NOT include duplicated parameters. A unique parameter is defined by a
    /// combination of a name and location. The list can use the Reference Object to link to
    /// parameters that are defined at the OpenAPI Object's components/parameters.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
    /// The request body applicable for this operation. The requestBody is only supported in HTTP
    /// methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics for
    /// request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored
    /// by consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RefOr<RequestBody>>,
    /// REQUIRED. The list of possible responses as they are returned from executing this operation.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<StringOrHttpCode, Response>,
    // A potentially recursive type, so boxed
    /// A map of possible out-of band callbacks related to the parent operation. The key is a
    /// unique identifier for the Callback Object. Each value in the map is a Callback Object that
    /// describes a request that may be initiated by the API provider and the expected responses.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub callbacks: HashMap<String, RefOr<HashMap<String, PathItem>>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage of the
    /// declared operation. Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// A declaration of which security mechanisms can be used for this operation. The list of
    /// values includes alternative security requirement objects that can be used. Only one of the
    /// security requirement objects need to be satisfied to authorize a request. To make security
    /// optional, an empty security requirement ({}) can be included in the array. This definition
    /// overrides any declared top-level security. To remove a top-level security declaration,
    /// an empty array can be used.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,
    /// An alternative server array to service this operation. If an alternative server object is
    /// specified at the Path Item Object or Root level, it will be overridden by this value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
