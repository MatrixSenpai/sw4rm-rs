#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLogEntryResponse {
    pub actiontype: !,
    pub reason: !,
    pub id: !,
}
