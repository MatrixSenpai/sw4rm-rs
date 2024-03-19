#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LobbyEventVfiveDto {
    pub timestamp: !,
    pub event_type: !,
    pub puuid: !,
}
