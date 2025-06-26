use crate::RefOr;
use crate::example::Example;
use crate::media_type::MediaType;
use crate::schema::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: ParameterLocation,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,

    pub style: Option<ParameterStyle>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<RefOr<Schema>>,
    pub example: Value,
    pub examples: HashMap<String, RefOr<Example>>,

    pub content: HashMap<String, MediaType>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ParameterLocation {
    #[default]
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ParameterStyle {
    Matrix,
    #[default]
    Label,
    Simple,
    Form,
    SpaceDelimited,
    PipeDelimited,
    DeepObject,
}
