#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyItemInventoryBlockDefinition {
    pub tier_type_name: !,
    pub bucket_type_hash: !,
    pub expired_in_orbit_message: !,
    pub stack_unique_label: !,
    pub max_stack_size: !,
    pub recipe_item_hash: !,
    pub recovery_bucket_type_hash: !,
    pub tier_type: !,
    pub suppress_expiration_when_objectives_complete: !,
    pub expired_in_activity_message: !,
    pub tier_type_hash: !,
    pub expiration_tooltip: !,
    pub is_instance_item: !,
}
