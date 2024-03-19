#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AbilityDto {
    pub abilitytwo_effects: !,
    pub abilityone_effects: !,
    pub ultimate_effects: !,
    pub grenade_effects: !,
}
