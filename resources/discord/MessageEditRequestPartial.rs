#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageEditRequestPartial {
    pub flags: !,
    pub content: !,
}
