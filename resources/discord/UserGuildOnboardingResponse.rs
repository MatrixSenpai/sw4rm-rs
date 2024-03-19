#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserGuildOnboardingResponse {
    pub guildid: !,
    pub enabled: !,
}
