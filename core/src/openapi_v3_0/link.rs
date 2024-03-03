use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Spec,
    reference::*,
    openapi_v3_0::Server,
};

/// Link Object
///
/// The Link object represents a possible design-time link for a response. The presence of a link
/// does not guarantee the caller's ability to successfully invoke it, rather it provides a known
/// relationship and traversal mechanism between responses and other operations.
///
/// Unlike dynamic links (i.e. links provided in the response payload), the OAS linking mechanism
/// does not require link information in the runtime response.
///
/// For computing links, and providing instructions to execute them, a runtime expression is used
/// for accessing values in an operation and using them as parameters while invoking the linked
/// operation.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation. This field is mutually exclusive
    /// of the operationId field, and MUST point to an Operation Object. Relative operationRef
    /// values MAY be used to locate an existing Operation Object in the OpenAPI definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation, as defined with a unique operationId.
    /// This field is mutually exclusive of the operationRef field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A map representing parameters to pass to an operation as specified with operationId or
    /// identified via operationRef. The key is the parameter name to be used, whereas the value
    /// can be a constant or an expression to be evaluated and passed to the linked operation.
    /// The parameter name can be qualified using the parameter location [{in}.]{name} for
    /// operations that use the same parameter name in different locations (e.g. path.id).
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, Value>,
    /// A literal value or {expression} to use as a request body when calling the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<Value>,
    /// A description of the link. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}

impl Resolvable for Link {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Link => {
                spec.components
                    .as_ref()
                    .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                    .and_then(|c| c.links.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                    .and_then(|p| p.resolve(spec))
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}