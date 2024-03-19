#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageMentionChannelResponse {
    pub id: !,
    pub name: !,
    pub type: !,
    pub guildid: !,
}
