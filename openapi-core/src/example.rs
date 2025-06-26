use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Example {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: Option<Value>,
    pub external_value: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
