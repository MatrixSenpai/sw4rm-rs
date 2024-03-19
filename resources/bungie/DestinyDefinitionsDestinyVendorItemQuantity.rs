#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyVendorItemQuantity {
    pub item_instance_id: !,
    pub item_hash: !,
    pub has_conditional_visibility: !,
    pub quantity: !,
}
