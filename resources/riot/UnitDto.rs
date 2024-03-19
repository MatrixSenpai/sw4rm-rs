#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UnitDto {
    pub rarity: !,
    pub tier: !,
    pub name: !,
    pub characterid: !,
    pub chosen: !,
}
