#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FriendInviteResponse {
    pub maxage: !,
    pub code: !,
    pub uses: !,
    pub expiresat: !,
    pub iscontact: !,
    pub friendscount: !,
    pub createdat: !,
    pub maxuses: !,
    pub flags: !,
}
