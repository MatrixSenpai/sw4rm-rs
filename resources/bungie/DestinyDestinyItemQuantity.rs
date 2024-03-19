#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDestinyItemQuantity {
    pub item_instance_id: !,
    pub item_hash: !,
    pub quantity: !,
    pub has_conditional_visibility: !,
}
