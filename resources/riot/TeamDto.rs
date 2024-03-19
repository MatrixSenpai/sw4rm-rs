#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TeamDto {
    pub num_points: !,
    pub rounds_played: !,
    pub won: !,
    pub rounds_won: !,
    pub team_id: !,
}
