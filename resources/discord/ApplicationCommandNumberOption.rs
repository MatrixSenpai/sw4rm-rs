#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandNumberOption {
    pub description: !,
    pub type: !,
    pub minvalue: !,
    pub maxvalue: !,
    pub autocomplete: !,
    pub name: !,
    pub required: !,
}
