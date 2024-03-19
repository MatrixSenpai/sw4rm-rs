#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BasicMessageResponse {
    pub content: !,
    pub channelid: !,
    pub mentioneveryone: !,
    pub position: !,
    pub id: !,
    pub timestamp: !,
    pub type: !,
    pub pinned: !,
    pub flags: !,
    pub tts: !,
    pub editedtimestamp: !,
}
