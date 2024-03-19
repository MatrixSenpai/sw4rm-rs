#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DefaultKeywordListUpsertRequest {
    pub triggertype: !,
    pub enabled: !,
    pub name: !,
    pub eventtype: !,
}
