#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandSubcommandOption {
    pub type: !,
    pub description: !,
    pub required: !,
    pub name: !,
}
