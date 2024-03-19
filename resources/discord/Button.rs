#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Button {
    pub type: !,
    pub label: !,
    pub disabled: !,
    pub url: !,
    pub customid: !,
    pub style: !,
}
