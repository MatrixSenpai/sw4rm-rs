#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyEntitiesItemsDestinyItemInstanceComponent {
    pub damage_type_hash: !,
    pub item_level: !,
    pub is_equipped: !,
    pub breaker_type: !,
    pub damage_type: !,
    pub can_equip: !,
    pub cannot_equip_reason: !,
    pub breaker_type_hash: !,
    pub quality: !,
    pub equip_required_level: !,
}
