#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Error {
    pub code: !,
    pub message: !,
}
