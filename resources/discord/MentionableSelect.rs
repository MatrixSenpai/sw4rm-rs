#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MentionableSelect {
    pub placeholder: !,
    pub disabled: !,
    pub minvalues: !,
    pub type: !,
    pub maxvalues: !,
    pub customid: !,
}
