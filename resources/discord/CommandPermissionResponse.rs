#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommandPermissionResponse {
    pub permission: !,
    pub id: !,
    pub type: !,
}
