#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CoachDto {
    pub team_id: !,
    pub puuid: !,
}
