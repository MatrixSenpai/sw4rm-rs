#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IncomingWebhookRequestPartial {
    pub tts: !,
    pub avatarurl: !,
    pub flags: !,
    pub threadname: !,
    pub username: !,
    pub content: !,
}
