#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PrivateGuildMemberResponse {
    pub deaf: !,
    pub banner: !,
    pub joinedat: !,
    pub avatar: !,
    pub nick: !,
    pub communicationdisableduntil: !,
    pub premiumsince: !,
    pub pending: !,
    pub flags: !,
    pub mute: !,
}
