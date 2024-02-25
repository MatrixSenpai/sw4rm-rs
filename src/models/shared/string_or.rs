use serde::{Deserialize, Serialize};
use crate::models::openapi_v3_0::Discriminator;

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum StringOrHttpCode {
    String(String),
    StatusCode(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum StringOrDiscriminator {
    String(String),
    Discriminator(Discriminator),
}