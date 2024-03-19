#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Error {
    pub message: !,
    pub code: !,
}
