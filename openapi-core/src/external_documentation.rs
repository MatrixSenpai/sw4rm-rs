use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
