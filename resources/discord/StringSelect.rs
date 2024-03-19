#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StringSelect {
    pub type: !,
    pub placeholder: !,
    pub customid: !,
    pub minvalues: !,
    pub maxvalues: !,
    pub disabled: !,
}
