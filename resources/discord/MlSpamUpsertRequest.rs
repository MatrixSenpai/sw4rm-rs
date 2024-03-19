#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MlSpamUpsertRequest {
    pub enabled: !,
    pub triggertype: !,
    pub name: !,
    pub eventtype: !,
}
