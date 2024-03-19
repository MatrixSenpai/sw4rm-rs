#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentUserSelectResponse {
    pub maxvalues: !,
    pub disabled: !,
    pub customid: !,
    pub placeholder: !,
    pub type: !,
    pub minvalues: !,
}
