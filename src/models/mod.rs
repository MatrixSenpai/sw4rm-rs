use serde::{Deserialize, Serialize};

mod swagger_v2;
mod swagger_v3_0;
mod shared;

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
