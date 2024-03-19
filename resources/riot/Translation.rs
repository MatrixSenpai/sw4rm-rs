#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Translation {
    pub locale: !,
    pub content: !,
    pub updatedat: !,
}
