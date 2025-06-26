use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Discriminator {
    pub property_name: String,
    pub mapping: HashMap<String, String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
