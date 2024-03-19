#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageDto {
    pub receiver: !,
    pub legshots: !,
    pub bodyshots: !,
    pub headshots: !,
    pub damage: !,
}
