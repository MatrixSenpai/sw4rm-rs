#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MentionSpamUpsertRequest {
    pub eventtype: !,
    pub enabled: !,
    pub triggertype: !,
    pub name: !,
}
