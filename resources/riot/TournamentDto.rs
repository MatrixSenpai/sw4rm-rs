#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentDto {
    pub name_key_secondary: !,
    pub id: !,
    pub name_key: !,
    pub theme_id: !,
}
