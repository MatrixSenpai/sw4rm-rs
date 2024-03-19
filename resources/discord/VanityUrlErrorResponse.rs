#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VanityUrlErrorResponse {
    pub code: !,
    pub message: !,
}
