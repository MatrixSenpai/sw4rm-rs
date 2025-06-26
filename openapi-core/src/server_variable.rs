use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    pub substitution_enum: Vec<String>,
    #[serde(rename = "default")]
    pub variable_default: String,
    pub description: String,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
