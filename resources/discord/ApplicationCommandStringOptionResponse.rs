#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandStringOptionResponse {
    pub descriptionlocalized: !,
    pub type: !,
    pub required: !,
    pub name: !,
    pub description: !,
    pub autocomplete: !,
    pub minlength: !,
    pub maxlength: !,
    pub namelocalized: !,
}
