#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyEntitiesProfilesDestinyProfileComponent {
    pub current_season_hash: !,
    pub date_last_played: !,
    pub versions_owned: !,
    pub current_season_reward_power_cap: !,
    pub active_event_card_hash: !,
    pub current_guardian_rank: !,
    pub lifetime_highest_guardian_rank: !,
}
