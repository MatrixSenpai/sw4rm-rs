use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Query,
    Header,
    Path,
    FormData,
    Body,
    Cookie
}
impl Default for ParameterLocation {
    fn default() -> Self { Self::Query }
}
