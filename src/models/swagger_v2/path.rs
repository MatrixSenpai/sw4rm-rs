use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use super::{
    parameter::Parameter,
    operation::Operation,
};

/// Path Item Object
///
/// Describes the operations available on a single path. A Path Item may be empty, due to ACL
/// constraints. The path itself is still exposed to the documentation viewer but they will not
/// know which operations and parameters are available.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be in
    /// the format of a Path Item Object. If there are conflicts between the referenced definition
    /// and this Path Item's definition, the behavior is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    /// A list of parameters that are applicable for all the operations described under this
    /// path. These parameters can be overridden at the operation level, but cannot be removed
    /// there. The list MUST NOT include duplicated parameters. A unique parameter is defined
    /// by a combination of a name and location. The list can use the Reference Object to link
    /// to parameters that are defined at the Swagger Object's parameters. There can be one
    /// "body" parameter at most.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}