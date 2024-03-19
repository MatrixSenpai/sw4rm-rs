#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ForumTagResponse {
    pub id: !,
    pub name: !,
    pub emojiname: !,
    pub moderated: !,
}
