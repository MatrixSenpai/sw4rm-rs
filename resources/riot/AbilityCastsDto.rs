#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AbilityCastsDto {
    pub grenade_casts: !,
    pub abilityone_casts: !,
    pub ultimate_casts: !,
    pub abilitytwo_casts: !,
}
