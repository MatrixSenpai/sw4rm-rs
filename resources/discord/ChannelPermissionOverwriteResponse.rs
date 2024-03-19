#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelPermissionOverwriteResponse {
    pub deny: !,
    pub allow: !,
    pub id: !,
    pub type: !,
}
