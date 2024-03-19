#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartialGuildSubscriptionIntegrationResponse {
    pub name: !,
    pub type: !,
    pub id: !,
}
