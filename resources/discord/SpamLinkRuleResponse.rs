#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpamLinkRuleResponse {
    pub id: !,
    pub triggertype: !,
    pub creatorid: !,
    pub eventtype: !,
    pub name: !,
    pub enabled: !,
    pub guildid: !,
}
