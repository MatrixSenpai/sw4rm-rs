#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserEmailOptInDefinition {
    pub set_by_default: !,
    pub name: !,
    pub value: !,
}
