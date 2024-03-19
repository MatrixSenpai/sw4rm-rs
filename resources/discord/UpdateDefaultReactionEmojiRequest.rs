#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateDefaultReactionEmojiRequest {
    pub emojiname: !,
}
