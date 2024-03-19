#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BannedChampion {
    pub team_id: !,
    pub champion_id: !,
    pub pick_turn: !,
}
