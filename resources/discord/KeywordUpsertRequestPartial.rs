#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KeywordUpsertRequestPartial {
    pub eventtype: !,
    pub triggertype: !,
    pub name: !,
    pub enabled: !,
}
