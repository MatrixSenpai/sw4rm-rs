#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandMentionableOption {
    pub name: !,
    pub required: !,
    pub description: !,
    pub type: !,
}
