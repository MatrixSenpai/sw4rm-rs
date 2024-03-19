#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SummonerDto {
    pub id: !,
    pub puuid: !,
    pub summoner_level: !,
    pub profile_icon_id: !,
    pub account_id: !,
    pub name: !,
    pub revision_date: !,
}
