use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{reference::*, shared::{
    Operation,
    Parameter,
}, openapi_v3_0::Server, Spec};

/// Path Item Object
///
/// Describes the operations available on a single path. A Path Item may be empty, due to ACL
/// constraints. The path itself is still exposed to the documentation viewer but they will not
/// know which operations and parameters are available.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct PathItem {
    // MARK: Common Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v3 Fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
}

impl Resolvable for PathItem {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::PathItem => {
                spec.paths.get(&reference.name)
                    .ok_or_else(|| ResolveError::UnknownPathError(path))
                    .and_then(|t| t.resolve(spec))
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}