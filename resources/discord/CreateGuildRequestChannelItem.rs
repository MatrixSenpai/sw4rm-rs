#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGuildRequestChannelItem {
    pub nsfw: !,
    pub position: !,
    pub userlimit: !,
    pub topic: !,
    pub rtcregion: !,
    pub defaultthreadratelimitperuser: !,
    pub ratelimitperuser: !,
    pub bitrate: !,
    pub name: !,
}
