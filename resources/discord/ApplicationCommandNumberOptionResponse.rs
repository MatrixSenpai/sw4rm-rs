#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandNumberOptionResponse {
    pub minvalue: !,
    pub description: !,
    pub name: !,
    pub descriptionlocalized: !,
    pub type: !,
    pub maxvalue: !,
    pub required: !,
    pub namelocalized: !,
    pub autocomplete: !,
}
