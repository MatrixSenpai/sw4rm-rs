#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerDto {
    pub orderofplay: !,
    pub puuid: !,
    pub gameoutcome: !,
    pub deckcode: !,
    pub deckid: !,
}
