#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGuildChannelRequest {
    pub userlimit: !,
    pub bitrate: !,
    pub name: !,
    pub defaultthreadratelimitperuser: !,
    pub ratelimitperuser: !,
    pub position: !,
    pub topic: !,
    pub nsfw: !,
    pub rtcregion: !,
}
