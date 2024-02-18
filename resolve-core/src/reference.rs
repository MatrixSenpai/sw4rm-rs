use serde::{Deserialize, Serialize};
use crate::Resolvable;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum RefOr<T> where T: Clone + Sized + Resolvable {
    Type(T),
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    }
}
