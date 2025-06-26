use crate::RefOr;
use crate::header::Header;
use crate::parameter::ParameterStyle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Encoding {
    pub content_type: String,
    pub headers: HashMap<String, RefOr<Header>>,
    pub style: Option<ParameterStyle>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
