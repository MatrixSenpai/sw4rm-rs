#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelSelect {
    pub maxvalues: !,
    pub disabled: !,
    pub minvalues: !,
    pub type: !,
    pub customid: !,
    pub placeholder: !,
}
