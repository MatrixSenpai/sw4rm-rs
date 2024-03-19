#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageCreateRequest {
    pub tts: !,
    pub content: !,
    pub flags: !,
}
