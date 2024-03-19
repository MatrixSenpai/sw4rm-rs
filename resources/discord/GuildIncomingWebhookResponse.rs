#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildIncomingWebhookResponse {
    pub token: !,
    pub avatar: !,
    pub url: !,
    pub id: !,
    pub name: !,
    pub type: !,
}
