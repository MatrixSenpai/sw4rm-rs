#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartialDiscordIntegrationResponse {
    pub type: !,
    pub name: !,
    pub applicationid: !,
    pub id: !,
}
