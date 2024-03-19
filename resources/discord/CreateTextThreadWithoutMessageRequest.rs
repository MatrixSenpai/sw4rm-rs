#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTextThreadWithoutMessageRequest {
    pub name: !,
    pub ratelimitperuser: !,
    pub invitable: !,
}
