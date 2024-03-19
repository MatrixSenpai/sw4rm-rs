#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandUserOptionResponse {
    pub name: !,
    pub type: !,
    pub description: !,
    pub descriptionlocalized: !,
    pub required: !,
    pub namelocalized: !,
}
