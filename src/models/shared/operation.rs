use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{reference::*, shared::{
    ExternalDocumentation,
    PathItem,
    Parameter,
    Response,
    StringOrHttpCode,
}, openapi_v2::Scheme, openapi_v3_0::{
    RequestBody,
    Server,
}, Spec};

/// Operation Object
///
/// Describes a single API operation on a path.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Operation {
    // MARK: Common Fields

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<StringOrHttpCode, RefOr<Response>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v2 Fields

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<Scheme>,

    // MARK: OpenAPI v3 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RefOr<RequestBody>>,
    // A potentially recursive type, so boxed
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub callbacks: HashMap<String, Callback>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Callback(RefOr<PathItem>);
impl Resolvable for Callback {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Callbacks => {
                spec.components
                    .as_ref()
                    .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                    .and_then(|c| c.callbacks.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                    .and_then(|p| p.resolve(spec))
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}