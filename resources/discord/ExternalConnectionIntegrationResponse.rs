#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExternalConnectionIntegrationResponse {
    pub name: !,
    pub type: !,
    pub syncedat: !,
    pub revoked: !,
    pub syncing: !,
    pub enableemoticons: !,
    pub id: !,
    pub subscribercount: !,
    pub enabled: !,
}
