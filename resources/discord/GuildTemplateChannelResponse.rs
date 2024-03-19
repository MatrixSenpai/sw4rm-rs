#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildTemplateChannelResponse {
    pub nsfw: !,
    pub themecolor: !,
    pub bitrate: !,
    pub id: !,
    pub defaultthreadratelimitperuser: !,
    pub name: !,
    pub userlimit: !,
    pub template: !,
    pub type: !,
    pub position: !,
    pub topic: !,
    pub ratelimitperuser: !,
}
