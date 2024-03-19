#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserPiiResponse {
    pub globalname: !,
    pub locale: !,
    pub bot: !,
    pub publicflags: !,
    pub system: !,
    pub banner: !,
    pub verified: !,
    pub accentcolor: !,
    pub mfaenabled: !,
    pub email: !,
    pub flags: !,
    pub username: !,
    pub avatar: !,
    pub discriminator: !,
    pub id: !,
}
