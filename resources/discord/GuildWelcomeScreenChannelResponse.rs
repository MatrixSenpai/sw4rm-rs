#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildWelcomeScreenChannelResponse {
    pub description: !,
    pub emojiname: !,
    pub channelid: !,
}
