#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandCreateRequest {
    pub defaultmemberpermissions: !,
    pub dmpermission: !,
    pub description: !,
    pub name: !,
}
