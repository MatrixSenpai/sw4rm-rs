#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateThreadTagRequest {
    pub emojiname: !,
    pub name: !,
    pub moderated: !,
}
