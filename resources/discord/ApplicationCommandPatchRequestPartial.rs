#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandPatchRequestPartial {
    pub description: !,
    pub name: !,
    pub defaultmemberpermissions: !,
    pub dmpermission: !,
}
