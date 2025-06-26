use crate::server_variable::ServerVariable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: HashMap<String, ServerVariable>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
