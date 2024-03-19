#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BanDto {
    pub pick_turn: !,
    pub champion_id: !,
}
