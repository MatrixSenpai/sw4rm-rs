#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateOrUpdateThreadTagRequest {
    pub emojiname: !,
    pub moderated: !,
    pub name: !,
}
