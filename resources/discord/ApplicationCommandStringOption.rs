#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandStringOption {
    pub autocomplete: !,
    pub minlength: !,
    pub description: !,
    pub maxlength: !,
    pub name: !,
    pub type: !,
    pub required: !,
}
