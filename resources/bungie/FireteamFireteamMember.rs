#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FireteamFireteamMember {
    pub has_microphone: !,
    pub last_platform_invite_attempt_date: !,
    pub date_joined: !,
    pub last_platform_invite_attempt_result: !,
    pub character_id: !,
}
