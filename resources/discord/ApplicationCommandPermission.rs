#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandPermission {
    pub id: !,
    pub type: !,
    pub permission: !,
}
