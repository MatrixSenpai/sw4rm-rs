use crate::RefOr;
use crate::operation::Operation;
use crate::parameter::Parameter;
use crate::server::Server;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

pub type Paths = HashMap<String, RefOr<PathItem>>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    pub servers: Vec<Server>,
    pub parameters: Vec<RefOr<Parameter>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
