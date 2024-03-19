#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CurrentGameParticipant {
    pub bot: !,
    pub profile_icon_id: !,
    pub spellone_id: !,
    pub champion_id: !,
    pub summoner_name: !,
    pub team_id: !,
    pub summoner_id: !,
    pub spelltwo_id: !,
    pub puuid: !,
}
