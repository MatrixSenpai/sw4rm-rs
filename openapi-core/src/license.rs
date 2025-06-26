use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct License {
    pub name: String,
    pub identifier: Option<String>,
    pub url: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
