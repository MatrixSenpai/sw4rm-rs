#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ThreadMetadataResponse {
    pub archived: !,
    pub archivetimestamp: !,
    pub autoarchiveduration: !,
    pub invitable: !,
    pub locked: !,
    pub createtimestamp: !,
}
