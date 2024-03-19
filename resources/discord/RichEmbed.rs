#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RichEmbed {
    pub timestamp: !,
    pub color: !,
    pub url: !,
    pub type: !,
    pub title: !,
    pub description: !,
}
