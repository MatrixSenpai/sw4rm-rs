#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IncomingWebhookUpdateRequestPartial {
    pub flags: !,
    pub content: !,
}
