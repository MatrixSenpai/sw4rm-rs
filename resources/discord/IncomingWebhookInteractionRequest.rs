#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IncomingWebhookInteractionRequest {
    pub content: !,
    pub tts: !,
    pub flags: !,
}
