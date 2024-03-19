#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildInviteResponse {
    pub temporary: !,
    pub maxuses: !,
    pub expiresat: !,
    pub maxage: !,
    pub approximatemembercount: !,
    pub flags: !,
    pub uses: !,
    pub approximatepresencecount: !,
    pub code: !,
    pub createdat: !,
    pub iscontact: !,
}
