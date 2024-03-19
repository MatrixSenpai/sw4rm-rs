#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BaseCreateMessageCreateRequest {
    pub content: !,
    pub flags: !,
}
