#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildOnboardingResponse {
    pub enabled: !,
    pub guildid: !,
}
