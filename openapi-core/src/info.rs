use std::collections::HashMap;
use crate::{RefOr, contact::Contact, license::License};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Info {
    pub title: String,
    pub summary: Option<String>,
    pub description: Option<RefOr<String>>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
