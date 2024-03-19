#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentChannelSelectResponse {
    pub minvalues: !,
    pub maxvalues: !,
    pub customid: !,
    pub disabled: !,
    pub placeholder: !,
    pub type: !,
}
