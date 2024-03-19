#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandResponse {
    pub type: !,
    pub descriptionlocalized: !,
    pub name: !,
    pub applicationid: !,
    pub nsfw: !,
    pub id: !,
    pub version: !,
    pub defaultmemberpermissions: !,
    pub dmpermission: !,
    pub namelocalized: !,
    pub description: !,
}
