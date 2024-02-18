use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use resolve_core::{ResolveRoot, Resolvable, ResolveError, RefType};
use crate::models::{
    Scheme,
    shared::{
        external_documentation::ExternalDocumentation,
        info::Info,
        tag::Tag,
    },
};
use super::{
    path::PathItem,
    parameter::Parameter,
    response::Response,
    schema::Schema,
    security::SecurityScheme,
};

/// Swagger Object
///
/// This is the root document object for the API specification. It combines what previously was
/// the Resource Listing and API Declaration (version 1.2 and earlier) together into one document.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Spec {
    /// Required. Specifies the Swagger Specification version being used. It can be used by
    /// the Swagger UI and other clients to interpret the API listing. The value MUST be "2.0".
    pub swagger: String,
    /// Required. Provides metadata about the API. The metadata can be used by the clients
    /// if needed.
    pub info: Info,
    /// The host (name or ip) serving the API. This MUST be the host only and does not include
    /// the scheme nor sub-paths. It MAY include a port. If the host is not included, the host
    /// serving the documentation is to be used (including the port). The host does not support
    /// path templating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The base path on which the API is served, which is relative to the host. If it is not
    /// included, the API is served directly under the host. The value MUST start with a leading
    /// slash (/). The basePath does not support path templating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// The transfer protocol of the API. Values MUST be from the list: "http", "https", "ws",
    /// "wss". If the schemes is not included, the default scheme to be used is Https
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<Scheme>,
    /// A list of MIME types the APIs can consume. This is global to all APIs but can be overridden
    /// on specific API calls. Value MUST be as described under Mime Types.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    /// A list of MIME types the APIs can produce. This is global to all APIs but can be overridden
    /// on specific API calls. Value MUST be as described under Mime Types.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    /// Required. The available paths and operations for the API.
    pub paths: HashMap<String, PathItem>,
    /// An object to hold data types produced and consumed by operations.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub definitions: HashMap<String, Schema>,
    /// An object to hold parameters that can be used across operations. This property does not
    /// define global parameters for all operations.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, Parameter>,
    /// An object to hold responses that can be used across operations. This property does not
    /// define global responses for all operations.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<String, Response>,
    /// Security scheme definitions that can be used across the specification.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub security_definitions: HashMap<String, SecurityScheme>,
    /// A declaration of which security schemes are applied for the API as a whole. The list of
    /// values describes alternative security schemes that can be used (that is, there is a
    /// logical OR between the security requirements). Individual operations can override
    /// this definition.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags
    /// can be used to reflect on their order by the parsing tools. Not all tags that are used by
    /// the Operation Object must be declared. The tags that are not declared may be organized
    /// randomly or based on the tools' logic. Each tag name in the list MUST be unique.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}

impl ResolveRoot for Spec {
    fn resolve<T: Resolvable>(&self, item_type: RefType, path: &str) -> Result<T, ResolveError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deserialize_yaml() {
        let file = std::fs::read_to_string("./resources/swagger_2_0.yaml").unwrap();
        let spec: super::Spec = serde_yaml::from_str(file.as_str()).unwrap();

        assert_eq!(
            spec.x_fields.get("x-internal-id"),
            Some(&serde_json::Value::String("test-item-exists".to_string()))
        )
    }

    #[test]
    fn test_deserialize_json() {
        let file = std::fs::read_to_string("./resources/riot-swaggerspec-2.0.json").unwrap();
        let _spec: super::Spec = serde_json::from_str(file.as_str()).unwrap();
    }
}