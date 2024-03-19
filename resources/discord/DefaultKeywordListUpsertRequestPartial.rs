#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DefaultKeywordListUpsertRequestPartial {
    pub enabled: !,
    pub eventtype: !,
    pub triggertype: !,
    pub name: !,
}
