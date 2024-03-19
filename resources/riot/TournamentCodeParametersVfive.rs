#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentCodeParametersVfive {
    pub map_type: !,
    pub metadata: !,
    pub team_size: !,
    pub enough_players: !,
    pub spectator_type: !,
    pub pick_type: !,
}
