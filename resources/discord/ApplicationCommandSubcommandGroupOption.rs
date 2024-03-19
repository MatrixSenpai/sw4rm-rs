#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandSubcommandGroupOption {
    pub required: !,
    pub name: !,
    pub type: !,
    pub description: !,
}
