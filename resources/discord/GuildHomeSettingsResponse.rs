#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildHomeSettingsResponse {
    pub enabled: !,
    pub guildid: !,
}
