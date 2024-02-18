use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::shared::{
    info::Info,
    external_documentation::ExternalDocumentation,
    tag::Tag,
};
use super::{
    server::Server,
    path::PathItem,
    components::Components,
};

/// OpenAPI Object
///
/// This is the root document object of the OpenAPI document.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Spec {
    /// REQUIRED. This string MUST be the semantic version number of the OpenAPI Specification
    /// version that the OpenAPI document uses. The openapi field SHOULD be used by tooling
    /// specifications and clients to interpret the OpenAPI document. This is not related to the
    /// API info.version string.
    pub openapi: String,
    /// REQUIRED. Provides metadata about the API. The metadata MAY be used by tooling as required.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server.
    /// If the servers property is not provided, or is an empty array, the default value would
    /// be a Server Object with a url value of /.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// REQUIRED. The available paths and operations for the API.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub paths: HashMap<String, PathItem>,
    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API. The list of values
    /// includes alternative security requirement objects that can be used. Only one of the
    /// security requirement objects need to be satisfied to authorize a request. Individual
    /// operations can override this definition. To make security optional, an empty security
    /// requirement ({}) can be included in the array.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags
    /// can be used to reflect on their order by the parsing tools. Not all tags that are used by
    /// the Operation Object must be declared. The tags that are not declared MAY be organized
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserializing_yaml() {
        let file = std::fs::read_to_string("./resources/openapi_3_0.yaml").unwrap();
        let spec: super::Spec = serde_yaml::from_str(file.as_str()).unwrap();

        assert_eq!(
            spec.x_fields.get("x-internal-id"),
            Some(&serde_json::Value::String("test-item-exists".to_string()))
        )
    }

    #[test]
    fn test_deserializing_json() {
        let file = std::fs::read_to_string("./resources/riot-openapi-3.0.0.json").unwrap();
        let _spec: super::Spec = serde_json::from_str(file.as_str()).unwrap();
    }
}