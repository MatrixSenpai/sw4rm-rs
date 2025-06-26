use crate::RefOr;
use crate::encoding::Encoding;
use crate::example::Example;
use crate::schema::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type Callback = HashMap<String, MediaType>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaType {
    pub schema: Option<RefOr<Schema>>,
    pub example: Value,
    pub examples: HashMap<String, RefOr<Example>>,
    pub encoding: HashMap<String, Encoding>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
