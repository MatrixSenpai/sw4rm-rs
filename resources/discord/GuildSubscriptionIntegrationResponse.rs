#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildSubscriptionIntegrationResponse {
    pub id: !,
    pub enabled: !,
    pub type: !,
    pub name: !,
}
