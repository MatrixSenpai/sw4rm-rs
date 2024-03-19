#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyItemSackBlockDefinition {
    pub vendor_sack_type: !,
    pub open_action: !,
    pub select_item_count: !,
    pub open_on_acquire: !,
    pub detail_action: !,
}
