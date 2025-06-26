use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct XML {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapped: Option<bool>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
