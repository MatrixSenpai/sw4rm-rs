#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StickerPackResponse {
    pub name: !,
    pub description: !,
    pub skuid: !,
    pub id: !,
}
