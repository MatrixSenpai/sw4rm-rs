use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{
    Spec,
    reference::*,
};

/// Example Object
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Long description for the example. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Embedded literal example. The value field and externalValue field are mutually exclusive.
    /// To represent examples of media types that cannot be naturally represented in JSON or YAML,
    /// use a string value to contain the example, escaping where necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// A URL that points to the literal example. This provides the capability to reference
    /// examples that cannot easily be included in JSON or YAML documents. The value field and
    /// externalValue field are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}

impl Resolvable for Example {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::Example => {
                spec.components
                    .as_ref()
                    .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                    .and_then(|c| c.examples.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                    .and_then(|p| p.resolve(spec))
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}