#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentStringSelectResponse {
    pub customid: !,
    pub minvalues: !,
    pub maxvalues: !,
    pub placeholder: !,
    pub type: !,
    pub disabled: !,
}
