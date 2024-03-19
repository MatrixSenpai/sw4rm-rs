#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildChannelResponse {
    pub nsfw: !,
    pub lastpintimestamp: !,
    pub rtcregion: !,
    pub type: !,
    pub userlimit: !,
    pub topic: !,
    pub guildid: !,
    pub bitrate: !,
    pub name: !,
    pub permissions: !,
    pub id: !,
    pub defaultthreadratelimitperuser: !,
    pub position: !,
    pub ratelimitperuser: !,
    pub flags: !,
}
