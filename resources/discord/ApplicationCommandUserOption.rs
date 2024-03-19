#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandUserOption {
    pub type: !,
    pub description: !,
    pub name: !,
    pub required: !,
}
