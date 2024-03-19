#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandChannelOption {
    pub description: !,
    pub required: !,
    pub name: !,
    pub type: !,
}
