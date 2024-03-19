#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentPhaseDto {
    pub id: !,
    pub registration_time: !,
    pub cancelled: !,
    pub start_time: !,
}
