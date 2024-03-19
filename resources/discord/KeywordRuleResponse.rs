#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KeywordRuleResponse {
    pub id: !,
    pub eventtype: !,
    pub triggertype: !,
    pub enabled: !,
    pub creatorid: !,
    pub guildid: !,
    pub name: !,
}
