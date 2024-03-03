use serde::{Deserialize, Serialize};

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

