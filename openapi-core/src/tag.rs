use std::collections::HashMap;
use crate::external_documentation::ExternalDocumentation;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
