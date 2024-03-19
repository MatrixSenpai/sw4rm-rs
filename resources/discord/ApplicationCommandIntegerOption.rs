#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandIntegerOption {
    pub required: !,
    pub type: !,
    pub autocomplete: !,
    pub description: !,
    pub name: !,
}
