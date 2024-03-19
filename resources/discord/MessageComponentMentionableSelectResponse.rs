#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentMentionableSelectResponse {
    pub maxvalues: !,
    pub type: !,
    pub customid: !,
    pub placeholder: !,
    pub minvalues: !,
    pub disabled: !,
}
