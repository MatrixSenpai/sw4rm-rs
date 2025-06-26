use crate::{RefOr, Reference, ReferenceKind, ReferenceLocation, Resolvable, Spec};
use crate::discriminator::Discriminator;
use crate::external_documentation::ExternalDocumentation;
use crate::xml::XML;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use anyhow::{bail, Context};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<SchemaTypeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<RefOr<Box<Self>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub additional_fields: HashMap<String, Value>,
}

impl Resolvable for Schema {
    fn resolve(spec: &Spec, reference: &Reference) -> anyhow::Result<Box<Self>> {
        match reference.location {
            ReferenceLocation::Components => {
                let inter = &spec.components;
                let v = match reference.kind {
                    Some(ReferenceKind::Schemas) => match inter {
                        Some(c) => &c.schemas,
                        None => bail!("Missing components!"),
                    },
                    _ => bail!("Unsupported!"),
                };
                
                let key = match &reference.name {
                    Some(v) => v,
                    None => bail!("Missing location!"),
                };
                
                v.get(key).map(|v| {
                    
                }).context("No item matching key")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Validate(bool),
    Item(RefOr<Box<Schema>>),
    Unknown(Value),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum SchemaTypeValue {
    Single(SchemaType),
    Multiple(Vec<SchemaType>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum SchemaType {
    Null,
    Array,
    Boolean,
    File,
    Integer,
    Number,
    Object,
    String,
}
