#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StandardStickerResponse {
    pub name: !,
    pub packid: !,
    pub tags: !,
    pub id: !,
    pub type: !,
    pub description: !,
    pub sortvalue: !,
}
