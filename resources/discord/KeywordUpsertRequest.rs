#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KeywordUpsertRequest {
    pub name: !,
    pub eventtype: !,
    pub enabled: !,
    pub triggertype: !,
}
