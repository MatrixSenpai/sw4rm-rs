#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelFollowerResponse {
    pub channelid: !,
    pub webhookid: !,
}
