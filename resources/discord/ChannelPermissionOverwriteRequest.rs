#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelPermissionOverwriteRequest {
    pub id: !,
    pub allow: !,
    pub deny: !,
}
