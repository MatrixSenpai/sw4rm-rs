#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentButtonResponse {
    pub disabled: !,
    pub url: !,
    pub customid: !,
    pub style: !,
    pub type: !,
    pub label: !,
}
