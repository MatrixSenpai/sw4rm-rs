#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Participant {
    pub champion_id: !,
    pub summoner_name: !,
    pub summoner_id: !,
    pub profile_icon_id: !,
    pub bot: !,
    pub team_id: !,
    pub spelltwo_id: !,
    pub puuid: !,
    pub spellone_id: !,
}
