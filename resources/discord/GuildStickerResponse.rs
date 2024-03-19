#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildStickerResponse {
    pub id: !,
    pub description: !,
    pub tags: !,
    pub name: !,
    pub available: !,
    pub guildid: !,
    pub type: !,
}
