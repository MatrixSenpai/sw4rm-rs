#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateGuildChannelRequestPartial {
    pub rtcregion: !,
    pub name: !,
    pub position: !,
    pub nsfw: !,
    pub bitrate: !,
    pub defaultthreadratelimitperuser: !,
    pub flags: !,
    pub ratelimitperuser: !,
    pub topic: !,
    pub userlimit: !,
}
