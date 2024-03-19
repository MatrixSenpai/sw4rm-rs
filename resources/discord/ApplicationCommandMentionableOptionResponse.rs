#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandMentionableOptionResponse {
    pub type: !,
    pub name: !,
    pub description: !,
    pub required: !,
    pub namelocalized: !,
    pub descriptionlocalized: !,
}
