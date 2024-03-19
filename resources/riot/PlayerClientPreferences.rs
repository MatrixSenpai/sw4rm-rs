#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerClientPreferences {
    pub banner_accent: !,
    pub crest_border: !,
    pub prestige_crest_border_level: !,
    pub title: !,
}
