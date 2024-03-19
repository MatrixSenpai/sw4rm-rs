#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PerkStatsDto {
    pub defense: !,
    pub flex: !,
    pub offense: !,
}
