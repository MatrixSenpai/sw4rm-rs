#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageResponse {
    pub mentioneveryone: !,
    pub flags: !,
    pub type: !,
    pub channelid: !,
    pub content: !,
    pub pinned: !,
    pub timestamp: !,
    pub editedtimestamp: !,
    pub position: !,
    pub id: !,
    pub tts: !,
}
