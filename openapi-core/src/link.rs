use crate::server::Server;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Link {
    pub operation_ref: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: HashMap<String, Value>,
    pub request_body: Option<Value>,
    pub description: Option<String>,
    pub server: Option<Server>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
