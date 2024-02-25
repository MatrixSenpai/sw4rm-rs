use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{
    Spec,
    reference::*,
    openapi_v3_0::media_type::MediaType,
};

/// Request Body Object
///
/// Describes a single request body.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of use. CommonMark
    /// syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// REQUIRED. The content of the request body. The key is a media type or media type range
    /// and the value describes it. For requests that match multiple keys, only the most specific
    /// key is applicable. e.g. text/plain overrides text/*
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
    /// Determines if the request body is required in the request. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}

impl Resolvable for RequestBody {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Link => {
                spec.components
                    .as_ref()
                    .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                    .and_then(|c| c.request_bodies.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                    .and_then(|p| p.resolve(spec))
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}