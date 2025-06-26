use crate::components::Components;
use crate::external_documentation::ExternalDocumentation;
use crate::path_item::PathItem;
use crate::security_requirement::SecurityRequirement;
use crate::tag::Tag;
use crate::{info::Info, path_item::Paths, server::Server};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Spec {
    pub openapi: String,
    pub info: Info,
    pub json_schema_dialect: Option<String>,
    pub servers: Vec<Server>,
    pub paths: Paths,
    pub webhooks: HashMap<String, PathItem>,
    pub components: Option<Components>,
    pub security: Vec<SecurityRequirement>,
    pub tags: Vec<Tag>,
    pub external_docs: Option<ExternalDocumentation>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
