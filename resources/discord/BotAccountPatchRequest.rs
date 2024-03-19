#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BotAccountPatchRequest {
    pub username: !,
    pub avatar: !,
}
