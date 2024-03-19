#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTextThreadWithMessageRequest {
    pub ratelimitperuser: !,
    pub name: !,
}
