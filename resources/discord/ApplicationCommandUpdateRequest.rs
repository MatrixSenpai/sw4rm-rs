#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandUpdateRequest {
    pub defaultmemberpermissions: !,
    pub name: !,
    pub description: !,
    pub dmpermission: !,
}
