#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommandPermissionsResponse {
    pub applicationid: !,
    pub guildid: !,
    pub id: !,
}
