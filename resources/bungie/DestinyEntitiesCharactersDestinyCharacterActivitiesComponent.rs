#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyEntitiesCharactersDestinyCharacterActivitiesComponent {
    pub last_completed_story_hash: !,
    pub current_activity_hash: !,
    pub current_activity_mode_hash: !,
    pub date_activity_started: !,
    pub current_activity_mode_type: !,
    pub current_playlist_activity_hash: !,
}
