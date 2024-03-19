#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MentionSpamUpsertRequestPartial {
    pub enabled: !,
    pub eventtype: !,
    pub triggertype: !,
    pub name: !,
}
