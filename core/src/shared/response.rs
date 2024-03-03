use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Spec,
    reference::*,
    shared::{
        Parameter,
        Schema,
    },
    openapi_v3_0::{
        Link,
        MediaType,
    },
};

/// Response Object
///
/// Describes a single response from an API Operation, including design-time, static links to
/// operations based on the response.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Response {
    // MARK: Common Fields

    pub description: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, RefOr<Parameter>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v2 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, Value>,

    // MARK: OpenAPI v3 Fields

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, RefOr<Link>>,
}

impl Resolvable for Response {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Response => {
                if spec.semver_check("=2") {
                    spec.responses.get(&reference.name)
                        .ok_or_else(|| ResolveError::UnknownPathError(path))
                        .and_then(|t| t.resolve(spec))
                } else {
                    spec.components
                        .as_ref()
                        .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                        .and_then(|c| c.responses.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                        .and_then(|p| p.resolve(spec))
                }
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}