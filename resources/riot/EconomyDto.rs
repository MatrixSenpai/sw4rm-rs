#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EconomyDto {
    pub weapon: !,
    pub loadout_value: !,
    pub armor: !,
    pub spent: !,
    pub remaining: !,
}
