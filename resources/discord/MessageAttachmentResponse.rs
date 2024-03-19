#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageAttachmentResponse {
    pub clipcreatedat: !,
    pub ephemeral: !,
    pub width: !,
    pub height: !,
    pub filename: !,
    pub durationsecs: !,
    pub title: !,
    pub url: !,
    pub contenttype: !,
    pub id: !,
    pub description: !,
    pub waveform: !,
    pub size: !,
    pub proxyurl: !,
}
