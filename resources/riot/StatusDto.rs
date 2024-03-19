#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusDto {
    pub id: !,
    pub createdat: !,
    pub incidentseverity: !,
    pub archiveat: !,
    pub maintenancestatus: !,
    pub updatedat: !,
}
