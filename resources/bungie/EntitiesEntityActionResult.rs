#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntitiesEntityActionResult {
    pub result: !,
    pub entity_id: !,
}
