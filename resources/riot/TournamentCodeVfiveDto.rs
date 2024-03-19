#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentCodeVfiveDto {
    pub code: !,
    pub password: !,
    pub provider_id: !,
    pub spectators: !,
    pub lobby_name: !,
    pub meta_data: !,
    pub pick_type: !,
    pub tournament_id: !,
    pub id: !,
    pub region: !,
    pub map: !,
    pub team_size: !,
}
