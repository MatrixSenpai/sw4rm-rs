use serde::{Deserialize, Serialize};

pub mod openapi_v2;
pub mod openapi_v3_0;
pub mod shared;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http, Https,
    Ws, Wss,
}
impl Default for Scheme {
    fn default() -> Self {
        Self::Https
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum StringOrHttpCode {
    String(String),
    StatusCode(u64),
}
