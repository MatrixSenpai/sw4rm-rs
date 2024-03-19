#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoundResultDto {
    pub round_num: !,
    pub round_result: !,
    pub winning_team: !,
    pub round_result_code: !,
    pub plant_site: !,
    pub bomb_defuser: !,
    pub defuse_round_time: !,
    pub round_ceremony: !,
    pub plant_round_time: !,
    pub bomb_planter: !,
}
