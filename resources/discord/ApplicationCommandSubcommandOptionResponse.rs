#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandSubcommandOptionResponse {
    pub required: !,
    pub description: !,
    pub descriptionlocalized: !,
    pub name: !,
    pub namelocalized: !,
    pub type: !,
}
