#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentRoleSelectResponse {
    pub maxvalues: !,
    pub disabled: !,
    pub minvalues: !,
    pub type: !,
    pub placeholder: !,
    pub customid: !,
}
