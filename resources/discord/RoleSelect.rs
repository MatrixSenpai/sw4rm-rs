#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleSelect {
    pub customid: !,
    pub disabled: !,
    pub maxvalues: !,
    pub type: !,
    pub placeholder: !,
    pub minvalues: !,
}
