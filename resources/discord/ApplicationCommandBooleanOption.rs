#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandBooleanOption {
    pub type: !,
    pub name: !,
    pub description: !,
    pub required: !,
}
