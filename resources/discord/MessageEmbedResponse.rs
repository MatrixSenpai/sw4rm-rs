#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageEmbedResponse {
    pub color: !,
    pub url: !,
    pub title: !,
    pub type: !,
    pub timestamp: !,
    pub description: !,
}
