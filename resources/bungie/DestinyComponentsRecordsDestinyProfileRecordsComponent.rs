#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyComponentsRecordsDestinyProfileRecordsComponent {
    pub score: !,
    pub active_score: !,
    pub lifetime_score: !,
    pub tracked_record_hash: !,
    pub record_seals_root_node_hash: !,
    pub legacy_score: !,
    pub record_categories_root_node_hash: !,
}
