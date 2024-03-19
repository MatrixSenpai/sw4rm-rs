#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserResponse {
    pub bot: !,
    pub flags: !,
    pub system: !,
    pub avatar: !,
    pub discriminator: !,
    pub globalname: !,
    pub id: !,
    pub username: !,
    pub banner: !,
    pub accentcolor: !,
    pub publicflags: !,
}
