#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateForumThreadRequest {
    pub ratelimitperuser: !,
    pub name: !,
}
