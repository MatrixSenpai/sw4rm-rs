#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinishingDamageDto {
    pub damage_type: !,
    pub is_secondary_fire_mode: !,
    pub damage_item: !,
}
