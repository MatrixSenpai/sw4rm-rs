#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MlSpamRuleResponse {
    pub enabled: !,
    pub eventtype: !,
    pub id: !,
    pub name: !,
    pub guildid: !,
    pub creatorid: !,
    pub triggertype: !,
}
