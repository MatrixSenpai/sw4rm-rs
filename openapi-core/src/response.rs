use crate::RefOr;
use crate::header::Header;
use crate::link::Link;
use crate::media_type::MediaType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum ResponseMapKey {
    #[serde(rename = "default")]
    Default,
    HttpNumber(usize),
    HttpString(String),
}

pub type Responses = HashMap<ResponseMapKey, RefOr<Response>>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Response {
    pub description: Option<String>,
    pub headers: HashMap<String, RefOr<Header>>,
    pub content: HashMap<String, MediaType>,
    pub links: HashMap<String, RefOr<Link>>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
