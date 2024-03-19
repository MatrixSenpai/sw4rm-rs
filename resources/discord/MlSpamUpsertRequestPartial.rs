#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MlSpamUpsertRequestPartial {
    pub triggertype: !,
    pub eventtype: !,
    pub name: !,
    pub enabled: !,
}
