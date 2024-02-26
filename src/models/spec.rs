use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use semver::{Version, VersionReq};

use super::{
    RefOr,
    shared::{
        ExternalDocumentation,
        Info,
        Parameter,
        PathItem,
        Response,
        Schema,
        SecurityScheme,
        Tag,
    },
    openapi_v2::Scheme,
    openapi_v3_0::{
        Server,
        Components,
    }
};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Spec {
    // MARK: Common Fields

    #[serde(alias = "swagger", alias = "openapi")]
    pub spec_version: String,
    pub info: Info,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub paths: HashMap<String, RefOr<PathItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v2 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<Scheme>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub definitions: HashMap<String, RefOr<Schema>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, RefOr<Parameter>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<String, RefOr<Response>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub security_definitions: HashMap<String, RefOr<SecurityScheme>>,

    // MARK: OpenAPI v3.1 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema_dialect: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub webhooks: HashMap<String, RefOr<PathItem>>,
}

impl Spec {
    pub(crate) fn semver_check(&self, version: &str) -> bool {
        let spec_version = Version::parse(self.spec_version.replace("2.0", "2.0.0").as_str()).unwrap();
        let version = VersionReq::parse(version).unwrap();
        version.matches(&spec_version)
    }

    pub fn schemas(&self) -> HashMap<String, RefOr<Schema>> {
        self.components
            .as_ref()
            .map(|c| c.schemas.clone())
            .unwrap_or(self.definitions.clone())
            .clone()
    }
}