#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StageInstanceResponse {
    pub channelid: !,
    pub guildid: !,
    pub topic: !,
    pub privacylevel: !,
    pub id: !,
    pub discoverabledisabled: !,
}
