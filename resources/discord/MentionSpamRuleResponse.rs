#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MentionSpamRuleResponse {
    pub enabled: !,
    pub eventtype: !,
    pub guildid: !,
    pub triggertype: !,
    pub creatorid: !,
    pub name: !,
    pub id: !,
}
