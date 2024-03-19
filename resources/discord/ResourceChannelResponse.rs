#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResourceChannelResponse {
    pub channelid: !,
    pub description: !,
    pub icon: !,
    pub title: !,
}
