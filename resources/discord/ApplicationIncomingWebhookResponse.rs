#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationIncomingWebhookResponse {
    pub name: !,
    pub type: !,
    pub id: !,
    pub avatar: !,
}
