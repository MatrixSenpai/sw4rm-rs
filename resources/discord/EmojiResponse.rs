#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EmojiResponse {
    pub requirecolons: !,
    pub animated: !,
    pub id: !,
    pub managed: !,
    pub available: !,
    pub name: !,
}
