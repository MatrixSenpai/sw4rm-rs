#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserEmailViewDefinitionSetting {
    pub name: !,
    pub set_by_default: !,
    pub opt_in_aggregate_value: !,
}
