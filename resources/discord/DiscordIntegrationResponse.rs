#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscordIntegrationResponse {
    pub type: !,
    pub enabled: !,
    pub name: !,
    pub id: !,
}
