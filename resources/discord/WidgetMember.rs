#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WidgetMember {
    pub selfmute: !,
    pub suppress: !,
    pub discriminator: !,
    pub username: !,
    pub deaf: !,
    pub mute: !,
    pub selfdeaf: !,
    pub avatarurl: !,
    pub id: !,
    pub status: !,
}
