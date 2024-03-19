#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildWelcomeChannel {
    pub channelid: !,
    pub emojiname: !,
    pub description: !,
}
