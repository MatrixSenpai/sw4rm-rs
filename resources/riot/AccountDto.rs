#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountDto {
    pub puuid: !,
    pub game_name: !,
    pub tag_line: !,
}
