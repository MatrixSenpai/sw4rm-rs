#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LeagueListDto {
    pub name: !,
    pub league_id: !,
    pub tier: !,
    pub queue: !,
}
