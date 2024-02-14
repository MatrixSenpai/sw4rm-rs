use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum RefOr<T> where T: Clone + Sized {
    Type(T),
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    }
}
