#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DefaultKeywordRuleResponse {
    pub creatorid: !,
    pub triggertype: !,
    pub name: !,
    pub eventtype: !,
    pub id: !,
    pub guildid: !,
    pub enabled: !,
}
