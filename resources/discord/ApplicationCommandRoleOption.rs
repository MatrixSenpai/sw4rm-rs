#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandRoleOption {
    pub type: !,
    pub description: !,
    pub name: !,
    pub required: !,
}
