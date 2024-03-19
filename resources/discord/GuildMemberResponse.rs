#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildMemberResponse {
    pub nick: !,
    pub premiumsince: !,
    pub flags: !,
    pub mute: !,
    pub deaf: !,
    pub communicationdisableduntil: !,
    pub joinedat: !,
    pub avatar: !,
    pub pending: !,
}
