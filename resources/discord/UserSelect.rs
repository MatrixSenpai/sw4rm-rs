#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserSelect {
    pub minvalues: !,
    pub type: !,
    pub maxvalues: !,
    pub customid: !,
    pub placeholder: !,
    pub disabled: !,
}
